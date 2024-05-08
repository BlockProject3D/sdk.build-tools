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

use std::path::{Path, PathBuf};

pub trait PathExt {
    fn join_option<P: AsRef<Path>>(&self, path: Option<P>) -> PathBuf;
}

impl PathExt for Path {
    fn join_option<P: AsRef<Path>>(&self, path: Option<P>) -> PathBuf {
        match path {
            None => self.into(),
            Some(v) => self.join(v)
        }
    }
}

macro_rules! builder_registry {
    ($($module: ident::$name: ident),*) => {
        $(mod $module;)*

        #[derive(serde::Deserialize, Debug, Copy, Clone)]
        #[serde(rename_all = "snake_case")]
        pub enum BuilderType {
            $($name),*
        }

        impl BuilderType {
            pub fn call(&self, context: &crate::builder::interface::Context, module: &crate::builder::interface::Module, paths: &mut Vec<std::path::PathBuf>) {
                match self {
                    $(BuilderType::$name => crate::core::run_builder::<$module::$name>(context, module, paths))*
                }
            }
        }
    };
}

pub(crate) use builder_registry;
