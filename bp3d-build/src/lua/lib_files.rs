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

use bp3d_lua::decl_lib_func;
use bp3d_lua::libs::Lib;
use bp3d_lua::util::Namespace;
use bp3d_lua::vm::function::types::RFunction;
use bp3d_lua::vm::table::Table;
use crate::lua::obj_path::PathOrString;

decl_lib_func! {
    fn read_text(path: PathOrString) -> std::io::Result<String> {
        std::fs::read_to_string(path.as_path())
    }
}

decl_lib_func! {
    fn write_text(path: PathOrString, data: &str) -> std::io::Result<()> {
        std::fs::write(path.as_path(), data)
    }
}

decl_lib_func! {
    fn copy(src_path: PathOrString, dst_path: PathOrString) -> std::io::Result<()> {
        if let Some(parent) = dst_path.as_path().parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(src_path.as_path(), dst_path.as_path()).map(|_| ())
    }
}

decl_lib_func! {
    fn symlink(src_path: PathOrString, dst_path: PathOrString) -> std::io::Result<()> {
        #[cfg(unix)]
        return std::os::unix::fs::symlink(src_path.as_path(), dst_path.as_path()).map(|_| ());
        #[cfg(windows)]
        return Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "symlink"));
    }
}

decl_lib_func! {
    fn clean(path: PathOrString) -> std::io::Result<()> {
        if path.as_path().exists() {
            std::fs::remove_dir_all(path.as_path())?;
        }
        std::fs::create_dir_all(path.as_path())?;
        Ok(())
    }
}

decl_lib_func! {
    fn exists(path: PathOrString) -> bool {
        path.as_path().exists()
    }
}

decl_lib_func! {
    fn list<'a>(vm: &Vm, path: PathOrString) -> std::io::Result<Table<'a>> {
        let mut tbl = Table::new(vm);
        let files = path.as_path().read_dir()?;
        for file in files {
            let file = file?;
            let path = file.path();
            let name = file.file_name();
            let ty = file.file_type()?;
            let mut subt = Table::with_capacity(vm, 0, 4);
            subt.set(c"path", crate::lua::obj_path::Path::from(path)).unwrap();
            subt.set(c"name", name.as_encoded_bytes()).unwrap();
            if ty.is_dir() {
                subt.set(c"type", "dir").unwrap();
            } else if ty.is_file() {
                subt.set(c"type", "file").unwrap();
            } else if ty.is_symlink() {
                subt.set(c"type", "symlink").unwrap();
            } else {
                subt.set(c"type", "other").unwrap();
            }
            tbl.push(subt).unwrap();
        }
        Ok(tbl)
    }
}

pub struct FilesLib;

impl Lib for FilesLib {
    const NAMESPACE: &'static str = "bp3d.build.files";

    fn load(&self, namespace: &mut Namespace) -> bp3d_lua::vm::Result<()> {
        namespace.add([
            ("readText", RFunction::wrap(read_text)),
            ("writeText", RFunction::wrap(write_text)),
            ("copy", RFunction::wrap(copy)),
            ("symlink", RFunction::wrap(symlink)),
            ("clean", RFunction::wrap(clean)),
            ("exists", RFunction::wrap(exists)),
            ("list", RFunction::wrap(list))
        ])
    }
}
