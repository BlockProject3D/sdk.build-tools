// Copyright (c) 2026, BlockProject 3D
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
use bp3d_lua::libs::files::{Files, SandboxPath};
use bp3d_lua::libs::Lib;
use bp3d_lua::libs::lua::Lua;
use bp3d_lua::libs::lua::require::{Provider, Source};
use bp3d_lua::libs::os::{Compat, Instant, Time};
use bp3d_lua::libs::util::Util;
use bp3d_lua::vm::closure::arc::Shared;
use bp3d_lua::vm::core::load::Script;
use bp3d_lua::vm::error::Error;
use bp3d_lua::vm::registry::core::Key;
use bp3d_lua::vm::RootVm;
use bp3d_lua::vm::Result;
use bp3d_lua::vm::table::Table;
use bp3d_lua::vm::userdata::UserDataImmutable;
use bp3d_lua::vm::value::any::AnyParam;
use bp3d_lua::vm::value::{FromLua, IntoLua};
use bp3d_lua::vm::value::types::Function;
use bp3d_os::assets::get_executable_path;
use bp3d_util::path::PathExt;
use crate::lua::lib_command::CommandLib;
use crate::lua::lib_files::FilesLib;
use crate::lua::obj_artifact::ObjArtifact;
use crate::lua::obj_list::ObjList;
use crate::system::{Context, Features};
use bp3d_lua::libs::files::chroot;
use bp3d_lua::libs::files::chroot::Permissions;

struct SourcePath(PathBuf);

impl SourcePath {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        SourcePath(path.into())
    }

    pub fn from_installed() -> Self {
        let exe = get_executable_path().unwrap();
        let mut path = exe.join("../usr/share/lua");
        debug!("path: {:?}", &path);
        if !path.exists() {
            path = exe.join("../../res/lua");
        }
        debug!("path: {:?}", &path);
        assert!(path.exists());
        SourcePath(path)
    }
}

impl Source for SourcePath {
    fn run(&self, vm: &bp3d_lua::vm::Vm, path: &str, _: &str) -> Result<AnyParam> {
        let path = path.replace(".", "/");
        let path = self.0.join(path);
        let path = path.ensure_extension("lua");
        debug!("Injecting lua file at path: {:?}", &path);
        vm.run(Script::from_path(path).map_err(|e| Error::Loader(e.to_string()))?)
    }
}

pub struct Vm {
    vm: RootVm,
    provider: Shared<Provider>,
    paths: Vec<PathBuf>,
    main_class: Option<Key<bp3d_lua::vm::registry::types::Table>>
}

impl Vm {
    pub fn add_source(&self, name: &str, source: impl Source + 'static) {
        self.provider.add_source(name.into(), source);
    }

    pub fn get(&self) -> &bp3d_lua::vm::Vm {
        &self.vm
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    pub fn new(path: &Path) -> Result<Vm> {
        let provider = Shared::new(Provider::new());
        let mut paths = Vec::new();
        debug!("Adding root bp3d lua path...");
        let src = SourcePath::from_installed();
        paths.push(src.0.clone());
        provider.add_source("bp3d".into(), src);
        debug!("Project root = {:?}", path);
        debug!("Project name = {:?}", path.file_name());
        if let Ok(path) = bp3d_os::fs::get_absolute_path(path) {
            if let Some(name) = path.file_name().map(|v| v.to_str()).flatten() {
                let path = path.join("bp3d-build");
                debug!({name}, "Adding project lua path: {:?}...", path);
                paths.push(path.clone());
                provider.add_source(name.into(), SourcePath::new(path));
            }
        }
        let vm = RootVm::new();
        Lua::new().provider(provider.clone()).build().register(&vm)?;
        chroot::set_chroot(&vm, path);
        chroot::set_access(&vm, "/", Permissions::R);
        chroot::set_access(&vm, "/target", Permissions::R | Permissions::W);
        chroot::set_access(&vm, "/bp3d-build", Permissions::R | Permissions::X);
        (Compat, Instant, Time).register(&vm)?;
        Util.register(&vm)?;
        Files.register(&vm)?;
        CommandLib.register(&vm)?;
        FilesLib.register(&vm)?;
        ObjArtifact.register(&vm)?;
        ObjList.register(&vm)?;
        vm.run_code(c"require = bp3d.lua.require")?;
        Ok(Vm {
            vm,
            provider,
            paths,
            main_class: None
        })
    }

    pub fn with_class<R: 'static>(&self, f: impl FnOnce(&bp3d_lua::vm::Vm, Table) -> Result<R>) -> Result<R> {
        self.vm.scope(|vm| {
            let class = self.main_class.as_ref().unwrap().push(vm);
            f(vm, class)
        })
    }

    pub fn call_main<'a>(&self, len: usize, args: impl Iterator<Item = (&'a str, &'a str)>) -> Result<()> {
        assert!(self.main_class.is_some());
        self.vm.scope(|vm| {
            let mut args2 = Table::with_capacity(vm, 0, len);
            for (k, v) in args {
                args2.set(k, v)?;
            }
            let class = self.main_class.as_ref().unwrap().push(vm);
            let f: Function = class.get(c"init")?;
            f.call((class.clone(), args2))
        })
    }

    fn _call<'a, A: IntoLua, R: FromLua<'a>>(class: Table<'a>, vm: &bp3d_lua::vm::Vm, f: &'a Function<'a>, context: &Context, target: &str, arg: A) -> Result<R> {
        let mut ctx = Table::with_capacity(vm, 0, 4);
        ctx.set(c"path", SandboxPath::from_path_unchecked(context.path))?;
        ctx.set(c"target", target)?;
        ctx.set(c"configuration", context.configuration)?;
        if let Features::List(features) = context.features {
            let mut features2 = Table::with_capacity(vm, features.len(), 0);
            for feature in features {
                features2.push(*feature)?;
            }
            ctx.set(c"features", features2)?;
        }
        f.call((class, ctx, arg))
    }

    fn _call2<'a, A: IntoLua, R: FromLua<'a>>(class: Table<'a>, vm: &bp3d_lua::vm::Vm, f: &'a Function<'a>, context: &Context, targets: &[&str], arg: A) -> Result<R> {
        let mut ctx = Table::with_capacity(vm, 0, 4);
        ctx.set(c"path", SandboxPath::from_path_unchecked(context.path))?;
        let mut targets2 = Table::with_capacity(vm, targets.len(), 0);
        for target in targets {
            targets2.push(*target)?;
        }
        ctx.set(c"targets", targets2)?;
        ctx.set(c"configuration", context.configuration)?;
        if let Features::List(features) = context.features {
            let mut features2 = Table::with_capacity(vm, features.len(), 0);
            for feature in features {
                features2.push(*feature)?;
            }
            ctx.set(c"features", features2)?;
        }
        f.call((class, ctx, arg))
    }

    pub fn call_userdata<R: 'static + UserDataImmutable + Clone>(&self, name: &str, context: &Context, target: &str) -> Result<R> {
        assert!(self.main_class.is_some());
        self.vm.scope(|vm| {
            let class = self.main_class.as_ref().unwrap().push(vm);
            let f: Function = class.get(name)?;
            let obj: &R = Self::_call(class.clone(), vm, &f, context, target, ())?;
            Ok(obj.clone())
        })
    }

    pub fn call_target_list<A: IntoLua>(&self, name: &str, context: &Context, targets: &[&str], arg: A) -> Result<()> {
        assert!(self.main_class.is_some());
        self.vm.scope(|vm| {
            let class = self.main_class.as_ref().unwrap().push(vm);
            let f: Function = class.get(name)?;
            Self::_call2(class.clone(), vm, &f, context, targets, arg)
        })
    }

    pub fn call_context<A: IntoLua>(&self, name: &str, context: &Context, target: &str, arg: A) -> Result<()> {
        assert!(self.main_class.is_some());
        self.vm.scope(|vm| {
            let class = self.main_class.as_ref().unwrap().push(vm);
            let f: Function = class.get(name)?;
            Self::_call(class.clone(), vm, &f, context, target, arg)
        })
    }

    pub fn find(&self, name: &str) -> Option<PathBuf> {
        for v in &self.paths {
            let path = v.join(name);
            debug!("Check lua file path: {:?}", path);
            if path.exists() {
                return Some(path);
            }
        }
        None
    }

    pub fn run(&mut self, script_path: &Path) -> Result<()> {
        assert!(self.main_class.is_none());
        self.vm.scope(|vm| {
            let cl: Table = vm.run(Script::from_path(script_path).map_err(|e| Error::Loader(e.to_string()))?)?;
            self.main_class = Some(Key::new(cl));
            Ok(())
        })
    }
}
