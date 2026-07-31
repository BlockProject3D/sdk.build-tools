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

use crate::lua::obj_artifact::Artifact;
use crate::system::artifact::{LibType, Type};
use bp3d_lua::libs::files::SandboxPath;
use bp3d_lua::libs::Lib;
use bp3d_lua::util::Namespace;
use bp3d_lua::vm::userdata::case::Camel;
use bp3d_lua::{decl_lib_func, decl_userdata, impl_userdata};
use bp3d_lua_codegen::{FromParam, LuaType};
use std::cell::RefCell;

#[derive(LuaType, FromParam)]
enum ArtifactType {
    Bin,
    Lib,
    Header,
    Config,
    Other,
}

decl_userdata!(#[derive(Clone)] pub struct List(RefCell<crate::system::artifact::List>));

impl From<crate::system::artifact::List> for List {
    fn from(value: crate::system::artifact::List) -> Self {
        List(RefCell::new(value))
    }
}

impl List {
    pub fn into_inner(self) -> crate::system::artifact::List {
        self.0.into_inner()
    }
}

decl_lib_func! {
    fn new() -> List {
        List(RefCell::new(crate::system::artifact::List::new()))
    }
}

impl_userdata! {
    impl List {
        fn add(this: &List, artifact: Option<&Artifact>) -> () {
            this.0.borrow_mut().add_if_some(artifact.map(crate::system::artifact::Artifact::from))
        }

        fn add_folder(this: &List, vm: &Vm, ty: ArtifactType, path: SandboxPath, name: &str) -> std::io::Result<()> {
            let ty = match ty {
                ArtifactType::Bin => Type::Bin,
                ArtifactType::Lib => Type::Lib(LibType::Dynamic),
                ArtifactType::Header => Type::Header,
                ArtifactType::Config => Type::Config,
                ArtifactType::Other => Type::Resource
            };
            let path = path.to_path(vm).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            this.0.borrow_mut().add_folder(ty, &*path, name)
        }

        fn add_folder_exclude(this: &List, vm: &Vm, ty: ArtifactType, path: SandboxPath, excluded: &str, name: &str) -> std::io::Result<()> {
            let ty = match ty {
                ArtifactType::Bin => Type::Bin,
                ArtifactType::Lib => Type::Lib(LibType::Dynamic),
                ArtifactType::Header => Type::Header,
                ArtifactType::Config => Type::Config,
                ArtifactType::Other => Type::Resource
            };
            let path = path.to_path(vm).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            this.0.borrow_mut().add_folder_exclude(ty, &*path, excluded, name)
        }
    }
    static {
        [fn new];
    }
}

pub struct ObjList;

impl Lib for ObjList {
    const NAMESPACE: &'static str = "bp3d.build";

    fn load(&self, namespace: &mut Namespace) -> bp3d_lua::vm::Result<()> {
        namespace.add_userdata::<List>(c"List", Camel)
    }
}
