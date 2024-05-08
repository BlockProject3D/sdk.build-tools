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

use crate::packager::interface::{Context, Package};

use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{Command};
use bp3d_build_common::output::Output;

pub trait CommandExt {
    fn ensure<E: Error + From<std::io::Error>>(&mut self, fail_value: E) -> Result<(), E>;

    fn output_string(&mut self) -> std::io::Result<String>;
}

impl CommandExt for Command {
    fn ensure<E: Error + From<std::io::Error>>(&mut self, fail_value: E) -> Result<(), E> {
        let res = self.status()?;
        if !res.success() {
            Err(fail_value)
        } else {
            Ok(())
        }
    }

    fn output_string(&mut self) -> std::io::Result<String> {
        Ok(String::from_utf8_lossy(&self.output()?.stdout).into())
    }
}

pub fn ensure_clean_directories<'a>(directories: impl IntoIterator<Item = &'a Path>) -> std::io::Result<()> {
    for dir in directories {
        if dir.exists() {
            std::fs::remove_dir_all(dir)?;
        }
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

impl<'a, P: Package> bp3d_build_common::finder::Context for Context<'a, P> {
    fn get_target_path(&self, target: &str) -> PathBuf {
        self.get_target_path(target)
    }

    fn get_outputs(&self) -> impl Iterator<Item = Output> {
        self.package.get_outputs()
    }
}

macro_rules! packager_registry {
    ($($module: ident::$name: ident),*) => {
        $(mod $module;)*

        #[derive(clap::ValueEnum, Debug, Copy, Clone)]
        pub enum PackagerType {
            $($name),*
        }

        impl PackagerType {
            pub fn call<P: crate::packager::interface::Package>(&self, context: &crate::packager::interface::Context<P>) {
                match self {
                    $(PackagerType::$name => crate::core::run_packager::<P, $module::$name>(context))*
                }
            }
        }
    };
}

pub(crate) use packager_registry;
