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

use std::path::PathBuf;
use mlua::{UserData, UserDataMethods};
use crate::builder::interface::OutputList;

pub struct OutputListWrapper(OutputList<'static>);

impl OutputListWrapper {
    pub fn new() -> Self {
        Self(OutputList::new())
    }

    pub fn into_inner(self) -> OutputList<'static> {
        self.0
    }
}

impl UserData for OutputListWrapper {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("addTargetPath", |_, this, path: String| {
            this.0.add_target_path(PathBuf::from(path));
            Ok(())
        });
        methods.add_method_mut("addBin", |_, this, name: String| {
            this.0.add_bin(name);
            Ok(())
        });
        methods.add_method_mut("addLib", |_, this, name: String| {
            this.0.add_lib(name);
            Ok(())
        });
        methods.add_method_mut("addConfig", |_, this, name: String| {
            this.0.add_config(name);
            Ok(())
        });
        methods.add_method_mut("addOther", |_, this, name: String| {
            this.0.add_other(name);
            Ok(())
        });
    }
}
