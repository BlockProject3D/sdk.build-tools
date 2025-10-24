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

use std::collections::HashMap;
use bp3d_util::simple_error;
use bp3d_build::system::artifact::List;
use crate::Context;
use crate::packager::interface::{build_target, Packager};

simple_error! {
    pub Error {
        (impl From) Lua(bp3d_lua::vm::error::Error) => "lua error: {}",
        Build(bp3d_build::core::Error) => "build error: {}"
    }
}

pub struct Lua<'a> {
    kvs: HashMap<String, String>,
    lua: Option<bp3d_build::lua::core::Vm>,
    context: &'a Context<'a>
}

impl<'a> Packager<'a> for Lua<'a> {
    const NAME: &'static str = "Lua";
    type Error = Error;
    type Config = HashMap<String, String>;

    fn new(config: Self::Config, _: &'a Context<'a>) -> Result<Self, Self::Error> {
        todo!()
    }

    fn do_build_target(&self, target: &str) -> Result<List, Self::Error> {
        build_target(&self.context, target).map_err(Error::Build)
    }

    fn do_build(&self) -> Result<(), Self::Error> {
        //self.lua = Some(bp3d_build::lua::core::Vm::new(context.path));
        todo!()
    }

    fn do_package_target(&self, _list: &List, _target: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn do_package(&self) -> Result<(), Self::Error> {
        todo!()
    }
}
