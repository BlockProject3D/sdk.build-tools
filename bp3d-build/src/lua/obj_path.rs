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

use std::path::PathBuf;
use bp3d_lua::{decl_lib_func, decl_userdata, impl_userdata};
use bp3d_lua::libs::Lib;
use bp3d_lua::util::Namespace;
use bp3d_lua::vm::userdata::case::Camel;
use bp3d_lua_codegen::{FromParam, LuaType};
use bp3d_util::path::PathExt;

#[derive(FromParam, LuaType)]
pub enum PathOrString<'a> {
    Path(&'a Path),
    String(&'a str),
}

impl<'a> PathOrString<'a> {
    pub fn as_path(&self) -> &std::path::Path {
        match self {
            PathOrString::Path(v) => v.as_path(),
            PathOrString::String(v) => v.as_ref()
        }
    }
}

decl_userdata!(pub struct Path(PathBuf));

impl From<PathBuf> for Path {
    fn from(path: PathBuf) -> Self {
        Path(path)
    }
}

impl From<&std::path::Path> for Path {
    fn from(value: &std::path::Path) -> Self {
        Path(value.into())
    }
}

impl Path {
    pub fn as_path(&self) -> &std::path::Path {
        &self.0
    }
}

decl_lib_func! {
    fn new(path: &str) -> Path {
        Path(PathBuf::from(path))
    }
}

impl_userdata! {
    impl Path {
        fn join(this: &Path, other: PathOrString) -> Path {
            let path = match other {
                PathOrString::Path(v) => this.0.join(v.as_path()),
                PathOrString::String(v) => this.0.join(v)
            };
            Path(path)
        }

        fn with_extension(this: &Path, extension: &str) -> Path {
            Path(this.0.ensure_extension(extension).into())
        }

        fn with_name(this: &Path, name: &str) -> Path {
            let mut path = this.0.clone();
            path.set_file_name(name);
            Path(path)
        }

        fn name(this: &Path) -> Option<String> {
            this.0.file_name().map(|v| v.to_string_lossy().into())
        }

        fn __tostring(this: &Path) -> String {
            this.0.display().to_string()
        }
    }

    static {
        [fn new];
    }
}

pub struct ObjPath;

impl Lib for ObjPath {
    const NAMESPACE: &'static str = "bp3d.build";

    fn load(&self, namespace: &mut Namespace) -> bp3d_lua::vm::Result<()> {
        namespace.add_userdata::<Path>("Path", Camel)
    }
}
