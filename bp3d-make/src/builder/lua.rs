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

use crate::builder::interface::{Builder, Context, Module, OutputList};
use crate::lua::engine::LuaEngine;

pub struct Lua {
    engine: LuaEngine
}

impl Builder for Lua {
    const NAME: &'static str = "Lua";
    type Error = mlua::Error;

    fn do_configure(context: &Context, module: &Module) -> Result<Self, Self::Error> {
        let engine = LuaEngine::new()?;
        engine.set_features(context.features.iter().map(|v| *v))?;
        engine.set_release(context.release)?;
        engine.set_target(context.target())?;
        engine.set_all_features(context.all_features)?;
        if module.path.extension()
            .map(|v| v.to_str().map(|v| v.to_lowercase() == "lua").unwrap_or(false))
            .unwrap_or(false) {
            engine.load_script(module.path)?;
        } else {
            engine.load_script(&module.path.join("bp3d-make.lua"))?;
        }
        engine.call("doConfigure")?;
        Ok(Lua {
            engine
        })
    }

    fn do_compile(&self, _: &Context, _: &Module) -> Result<(), Self::Error> {
        self.engine.call("doCompile")?;
        Ok(())
    }

    fn list_outputs(&self, _: &Context, _: &Module) -> Result<OutputList, Self::Error> {
        self.engine.call_outputs("listOutputs")
    }
}
