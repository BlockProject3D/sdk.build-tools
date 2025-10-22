// Copyright (c) 2025, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::path::{Path, PathBuf};
use bp3d_debug::debug;
use bp3d_lua::libs::Lib;
use bp3d_lua::libs::lua::Lua;
use bp3d_lua::libs::lua::require::{Provider, Source};
use bp3d_lua::libs::os::{Compat, Instant, Time};
use bp3d_lua::libs::util::Util;
use bp3d_lua::util::core::AnyStr;
use bp3d_lua::vm::closure::arc::Shared;
use bp3d_lua::vm::core::load::Script;
use bp3d_lua::vm::error::Error;
use bp3d_lua::vm::RootVm;
use bp3d_lua::vm::Result;
use bp3d_lua::vm::value::any::AnyParam;
use bp3d_lua::vm::value::{FromLua, IntoLua};
use bp3d_lua::vm::value::types::Function;
use bp3d_os::assets::get_executable_path;
use crate::lua::lib_command::CommandLib;
use crate::lua::lib_files::FilesLib;
use crate::lua::obj_artifact::ObjArtifact;
use crate::lua::obj_path::ObjPath;

struct SourcePath(PathBuf);

impl SourcePath {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        SourcePath(path.into())
    }

    pub fn from_installed() -> Self {
        let exe = get_executable_path().unwrap();
        let mut path = exe.join("../share/lua");
        if !path.exists() {
            path = exe.join("../../share/lua");
        }
        assert!(path.exists());
        SourcePath(path)
    }
}

impl Source for SourcePath {
    fn run(&self, vm: &bp3d_lua::vm::Vm, path: &str, _: &str) -> Result<AnyParam> {
        let path = path.replace(".", "/");
        vm.run(Script::from_path(self.0.join(path)).map_err(|e| Error::Loader(e.to_string()))?)
    }
}

pub struct Vm {
    vm: RootVm,
    provider: Shared<Provider>,
}

impl Vm {
    pub fn add_source(&self, name: &str, source: impl Source + 'static) {
        self.provider.add_source(name.into(), source);
    }

    pub fn new(path: &Path) -> Result<Vm> {
        let provider = Shared::new(Provider::new());
        debug!("Adding root bp3d lua path...");
        provider.add_source("bp3d".into(), SourcePath::from_installed());
        if let Some(name) = path.file_name().map(|v| v.to_str()).flatten() {
            debug!({name}, "Adding project lua path: {:?}...", path);
            provider.add_source(name.into(), SourcePath::new(path));
        }
        let vm = RootVm::new();
        Lua::new().provider(provider.clone()).load_chroot_path(path).build().register(&vm)?;
        (Compat, Instant, Time).register(&vm)?;
        Util.register(&vm)?;
        CommandLib.register(&vm)?;
        ObjPath.register(&vm)?;
        FilesLib.register(&vm)?;
        ObjArtifact.register(&vm)?;
        Ok(Vm {
            vm,
            provider
        })
    }

    pub fn run(&self, script_path: &Path) -> Result<()> {
        self.vm.scope(|vm| {
            vm.run(Script::from_path(script_path).map_err(|e| Error::Loader(e.to_string()))?)
        })
    }

    pub fn call<R: for <'a> FromLua<'a>, R2: 'static>(&self, name: impl AnyStr, arg: impl IntoLua, done: impl FnOnce(R) -> R2) -> Result<R2> {
        self.vm.scope(|vm| {
            let f: Function = vm.get_global(name)?;
            let r: R = f.call(arg)?;
            let r2 = done(r);
            Ok(r2)
        })
    }
}
