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

use std::collections::HashMap;
use bp3d_lua::libs::files::SandboxPath;
use bp3d_lua::vm::closure::types::RClosure;
use bp3d_lua::vm::table::Table;
use bp3d_lua::vm::value::types::Function;
use bp3d_lua::vm::Vm;
use bp3d_util::simple_error;
use bp3d_build::lua::core::dump_backtrace;
use bp3d_build::system::artifact::List;
use bp3d_build::system::Features;
use crate::packager::Context;
use crate::packager::interface::{build_target, Packager};
use bp3d_build::lua::List as LuaList;

simple_error! {
    pub Error {
        (impl From) Lua(bp3d_lua::vm::error::Error) => "lua error: {}",
        Build(bp3d_build::core::Error) => "build error: {}",
        NotFound(String) => "packager not found: {}"
    }
}

pub struct Lua<'a> {
    vm: bp3d_build::lua::core::Vm,
    context: &'a Context<'a>
}

fn create_context<'a>(vm: &'a Vm, context: &Context) -> bp3d_lua::vm::Result<Table<'a>> {
    let mut tbl = Table::with_capacity(vm, 0, 3);
    tbl.set(c"path", SandboxPath::from_path_unchecked(context.path))?;
    tbl.set(c"configuration", context.configuration)?;
    let mut package = Table::with_capacity(vm, 0, 2);
    package.set(c"name", context.tool.package().get_primary_name())?;
    package.set(c"version", context.tool.package().get_primary_version())?;
    if context.tool.package().get_components() > 0 {
        let mut components = Table::with_capacity(vm, 0, context.tool.package().get_components());
        for i in 0..context.tool.package().get_components() {
            let mut component = Table::with_capacity(vm, 0, 3);
            let c = context.tool.package().get_component(i);
            component.set("name", c.get_name())?;
            component.set("version", c.get_version())?;
            component.set("description", c.get_description())?;
            components.set(c.get_short_name(), component)?;
        }
        package.set("components", components)?;
    }
    tbl.set(c"package", package)?;
    let mut targets = Table::with_capacity(vm, context.targets.len(), 0);
    for target in context.targets {
        targets.push(*target)?;
    }
    tbl.set(c"targets", targets)?;
    Ok(tbl)
}

impl<'a> Packager<'a> for Lua<'a> {
    const NAME: &'static str = "Lua";
    type Error = Error;
    type Config = HashMap<String, String>;

    #[allow(dependency_on_unit_never_type_fallback)]
    fn new(config: Self::Config, context: &'a Context<'a>) -> Result<Self, Self::Error> {
        let mut vm = bp3d_build::lua::core::Vm::new(context.path)?;
        let path = vm.find(&format!("package/{}.lua", context.packager));
        if path.is_none() {
            return Err(Error::NotFound(context.packager.into()));
        }
        let path = path.unwrap();
        vm.run(&path)?;
        vm.call_main(config.len(), config.iter().map(|(k, v)| (&**k, &**v)))?;
        vm.with_class(|vm, class| {
            let f: Function = class.get(c"init2")?;
            let ctx = create_context(vm, context)?;
            dump_backtrace(f.call((class.clone(), ctx)))
        }).map_err(Error::Lua)?;
        Ok(Lua {
            context,
            vm
        })
    }

    fn do_build_target(&self, target: &str) -> Result<List, Self::Error> {
        let flag = self.vm.with_class(|_, class| {
            let f: Option<Function> = class.get(c"buildTarget")?;
            Ok(f.is_some())
        })?;
        if flag {
            let ctx = bp3d_build::system::Context {
                path: self.context.path,
                configuration: self.context.configuration,
                features: Features::All
            };
            let (f, _guard) = RClosure::from_rust_temporary(self.vm.get(), |config: Table| {
                let target: &str = config.get(c"target")?;
                build_target(&self.context, target).map(|v| LuaList::from(v)).map_err(Error::Build)
            });
            self.vm.get().set_global(c"baseBuild", f)?;
            let value: LuaList = self.vm.call_userdata("buildTarget", &ctx, target).map_err(Error::Lua)?;
            Ok(value.into_inner())
        } else {
            build_target(&self.context, target).map_err(Error::Build)
        }
    }

    fn do_build(&self) -> Result<(), Self::Error> {
        self.vm.with_class(|_, class| {
            let f: Function = class.get(c"build")?;
            dump_backtrace(f.call(class.clone()))
        }).map_err(Error::Lua)
    }

    fn do_package_target(&self, list: &List, target: &str) -> Result<(), Self::Error> {
        let ctx = bp3d_build::system::Context {
            path: self.context.path,
            configuration: self.context.configuration,
            features: Features::All
        };
        self.vm.call_context("packageTarget", &ctx, target, list.clone()).map_err(Error::Lua)
    }

    fn do_package(&self) -> Result<(), Self::Error> {
        self.vm.with_class(|_, class| {
            let f: Function = class.get(c"package")?;
            dump_backtrace(f.call(class.clone()))
        }).map_err(Error::Lua)
    }
}
