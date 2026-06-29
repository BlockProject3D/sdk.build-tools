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

use bp3d_lua::{decl_lib_func, decl_userdata, impl_userdata};
use bp3d_lua::libs::files::chroot::SandboxError;
use bp3d_lua::libs::files::SandboxPath;
use bp3d_lua::libs::Lib;
use bp3d_lua::util::Namespace;
use bp3d_lua::vm::table::Table;
use bp3d_lua::vm::userdata::case::Camel;
use bp3d_lua::vm::value::IntoLua;
use bp3d_lua::vm::Vm;
use bp3d_lua_codegen::{FromParam, LuaType};
use crate::system::artifact;
use crate::system::artifact::{LibType, List, Type};

decl_userdata!(pub struct Artifact(artifact::Artifact));

impl From<artifact::Artifact> for Artifact {
    fn from(artifact: artifact::Artifact) -> Self {
        Self(artifact)
    }
}

impl From<Artifact> for artifact::Artifact {
    fn from(value: Artifact) -> Self {
        value.0
    }
}

impl From<&Artifact> for artifact::Artifact {
    fn from(value: &Artifact) -> Self {
        value.0.clone()
    }
}

#[derive(LuaType, FromParam)]
enum LuaLibType {
    Dynamic,
    Static
}

decl_lib_func! {
    fn find_bin(vm: &Vm, path: SandboxPath, name: &str, remove_debug_info: Option<bool>) -> Result<Option<Artifact>, SandboxError> {
        path.to_path(vm).map(|v| artifact::Artifact::find_bin(&*v, name, remove_debug_info.unwrap_or_default()).map(Artifact))
    }
}

decl_lib_func! {
    fn find_lib(vm: &Vm, path: SandboxPath, name: &str, ty: LuaLibType, remove_debug_info: Option<bool>) -> Result<Option<Artifact>, SandboxError> {
        match ty {
            LuaLibType::Dynamic => path.to_path(vm).map(|v| artifact::Artifact::find_lib(&*v, name, LibType::Dynamic, remove_debug_info.unwrap_or_default()).map(Artifact)),
            LuaLibType::Static => path.to_path(vm).map(|v| artifact::Artifact::find_lib(&*v, name, LibType::Static, remove_debug_info.unwrap_or_default()).map(Artifact)),
        }
    }
}

decl_lib_func! {
    fn header(vm: &Vm, path: SandboxPath, name: &str) -> Result<Artifact, SandboxError> {
        path.to_path(vm).map(|v| Artifact(artifact::Artifact::header(&*v, name)))
    }
}

decl_lib_func! {
    fn config(vm: &Vm, path: SandboxPath, name: &str) -> Result<Artifact, SandboxError> {
        path.to_path(vm).map(|v| Artifact(artifact::Artifact::config(&*v, name)))
    }
}

decl_lib_func! {
    fn resource(vm: &Vm, path: SandboxPath, name: &str) -> Result<Artifact, SandboxError> {
        path.to_path(vm).map(|v| Artifact(artifact::Artifact::resource(&*v, name)))
    }
}

impl_userdata! {
    impl Artifact {
        fn path(this: &Artifact) -> SandboxPath {
            SandboxPath::from_path_unchecked(this.0.path())
        }

        fn debug_info(this: &Artifact) -> Option<SandboxPath> {
            this.0.debug_info().map(SandboxPath::from_path_unchecked)
        }

        fn exports(this: &Artifact) -> Option<SandboxPath> {
            this.0.exports().map(SandboxPath::from_path_unchecked)
        }

        fn name(this: &Artifact) -> &str {
            this.0.name()
        }

        fn ty(this: &Artifact) -> &'static str {
            match this.0.ty() {
                Type::Bin => "bin",
                Type::Lib(v) => match v {
                    LibType::Dynamic => "lib::dynamic",
                    LibType::Static => "lib::static",
                }
                Type::Header => "header",
                Type::Config => "config",
                Type::Resource => "resource"
            }
        }

        fn __tostring(this: &Artifact) -> &str {
            this.0.name()
        }
    }
    static {
        [fn find_bin];
        [fn find_lib];
        [fn header];
        [fn config];
        [fn resource];
    }
}

pub struct ObjArtifact;

impl Lib for ObjArtifact {
    const NAMESPACE: &'static str = "bp3d.build";

    fn load(&self, namespace: &mut Namespace) -> bp3d_lua::vm::Result<()> {
        namespace.add_userdata::<Artifact>("Artifact", Camel)
    }
}

unsafe impl IntoLua for List {
    fn into_lua(self, vm: &Vm) -> u16 {
        let inner = self.into_inner();
        let mut tbl = Table::with_capacity(vm, inner.len(), 0);
        for artifact in inner {
            tbl.push(Artifact(artifact)).unwrap();
        }
        1
    }
}
