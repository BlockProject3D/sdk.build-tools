// Copyright (c) 2024, BlockProject 3D
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


use std::path::Path;
use mlua::{FromLua, Function, Lua, LuaOptions, StdLib, Table};
use crate::lua::lib_command::CommandLib;

pub trait Lib {
    const NAME: &'static str;

    fn load(&self, lua: &Lua, table: &Table) -> mlua::Result<()>;
}

pub struct LuaEngine {
    lua: Lua
}

impl LuaEngine {
    pub fn load_lib<L: Lib>(&self, lib: L) -> mlua::Result<()> {
        let table = self.lua.create_table()?;
        lib.load(&self.lua, &table)?;
        self.lua.globals().set(L::NAME, table)?;
        Ok(())
    }

    pub fn load_script(&self, path: &Path) -> mlua::Result<()> {
        let data = std::fs::read_to_string(path)?;
        self.lua.load(data).set_name("Module Script").exec()?;
        Ok(())
    }

    pub fn new() -> mlua::Result<LuaEngine> {
        let lua = Lua::new_with(StdLib::ALL_SAFE, LuaOptions::new())?;
        let engine = LuaEngine { lua };
        engine.load_lib(CommandLib)?;
        Ok(engine)
    }

    pub fn set_target(&self, target: &str) -> mlua::Result<()> {
        self.lua.globals().set("BP3D_MAKE_TARGET", target)
    }

    pub fn set_all_features(&self, all_features: bool) -> mlua::Result<()> {
        self.lua.globals().set("BP3D_MAKE_FEATURE_ALL", all_features)?;
        if all_features {
            self.lua.globals().set("BP3D_MAKE_FEATURES", ["all"])?
        }
        Ok(())
    }

    pub fn set_features<'a>(&self, features: impl Iterator<Item = &'a str>) -> mlua::Result<()> {
        let mut motherfuckingrust = Vec::new();
        for feature in features {
            motherfuckingrust.push(feature);
            self.lua.globals().set(String::from("BP3D_MAKE_FEATURE_") + &feature.to_uppercase(), true)?;
        }
        self.lua.globals().set("BP3D_MAKE_FEATURES", motherfuckingrust)?;
        Ok(())
    }

    pub fn set_release(&self, release: bool) -> mlua::Result<()> {
        self.lua.globals().set("BP3D_MAKE_RELEASE", release)
    }

    pub fn call(&self, name: &str) -> mlua::Result<()> {
        let function = Function::from_lua(self.lua.globals().get(name)?, &self.lua)?;
        function.call(())?;
        Ok(())
    }
}
