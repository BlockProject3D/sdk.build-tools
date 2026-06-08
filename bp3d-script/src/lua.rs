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

use std::fmt::{Display, Formatter};
use bp3d_lua::libs::files::SandboxPath;
use bp3d_lua::vm::table::Table;
use bp3d_lua::vm::value::types::Function;
use bp3d_build::lua::core::{dump_backtrace, Vm};
use bp3d_build::lua::util::convert_package;
use bp3d_build::system::Features;
use crate::interface::{Context, Script};

#[derive(Debug)]
pub enum Error {
    Lua(bp3d_lua::vm::error::Error),
    NotFound(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Lua(e) => write!(f, "lua error: {}", e),
            Error::NotFound(name) => write!(f, "script not found ({})", name),
        }
    }
}

impl std::error::Error for Error {}

pub struct Lua {
    vm: Vm
}

fn create_context<'a>(vm: &'a bp3d_lua::vm::Vm, context: &Context) -> bp3d_lua::vm::Result<Table<'a>> {
    let mut tbl = Table::with_capacity(vm, 0, 5);
    tbl.set(c"path", SandboxPath::from_path_unchecked(context.path))?;
    tbl.set(c"configuration", context.configuration)?;
    tbl.set(c"package", convert_package(vm, context.tool.package())?)?;
    let mut targets = Table::with_capacity(vm, context.targets.len(), 0);
    for target in context.targets {
        targets.push(*target)?;
    }
    tbl.set(c"targets", targets)?;
    match context.features {
        Features::All => (),
        Features::List(features) => {
            let mut tbl1 = Table::with_capacity(vm, features.len(), 0);
            for feature in features {
                tbl1.push(*feature)?;
            }
            tbl.set(c"features", tbl1)?;
        }
    }
    Ok(tbl)
}

impl Script for Lua {
    type Error = Error;

    #[allow(dependency_on_unit_never_type_fallback)]
    fn new(context: &Context, name: &str, args: &[&str]) -> Result<Self, Self::Error> {
        let mut vm = Vm::new(context.path).map_err(Error::Lua)?;
        let path = vm.find(&format!("script/{}.lua", name));
        if path.is_none() {
            return Err(Error::NotFound(name.into()));
        }
        let path = path.unwrap();
        vm.run(&path).map_err(Error::Lua)?;
        if args.is_empty() {
            vm.call_main(0, [].into_iter()).map_err(Error::Lua)?;
        } else {
            let args = args.iter().map(|v| {
                match v.find('=') {
                    Some(pos) => (&v[..pos], &v[pos + 1..]),
                    None => (*v, "")
                }
            });
            vm.call_main(args.len(), args).map_err(Error::Lua)?;
        }
        vm.with_class(|vm, class| {
            let f: Function = class.get(c"init2")?;
            let ctx = create_context(vm, context)?;
            dump_backtrace(f.call((class.clone(), ctx)))
        }).map_err(Error::Lua)?;
        Ok(Lua {
            vm
        })
    }

    fn needs_configure(&self) -> Result<bool, Self::Error> {
        self.vm.with_class(|_, class| {
            let f: Function = class.get(c"needsConfigure")?;
            dump_backtrace(f.call(class.clone()))
        }).map_err(Error::Lua)
    }

    fn needs_build(&self) -> Result<bool, Self::Error> {
        self.vm.with_class(|_, class| {
            let f: Function = class.get(c"needsBuild")?;
            dump_backtrace(f.call(class.clone()))
        }).map_err(Error::Lua)
    }

    fn execute(&self) -> Result<(), Self::Error> {
        self.vm.with_class(|_, class| {
            let f: Function = class.get(c"run")?;
            dump_backtrace(f.call(class.clone()))
        }).map_err(Error::Lua)
    }
}
