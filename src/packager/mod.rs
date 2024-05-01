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

mod framework;

pub use framework::Framework;

use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use cargo_toml::Manifest;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Config {
    Debug,
    Release
}

pub struct Context<'a> {
    pub root: &'a Path,
    pub manifest: Manifest,
    pub config: Config,
    pub targets: &'a [&'a str]
}

impl<'a> Context<'a> {
    pub fn get_target_path(&self, target: &str) -> PathBuf {
        let config_path_name = match self.config {
            Config::Debug => "debug",
            Config::Release => "release"
        };
        self.root.join("target").join(target).join(config_path_name)
    }
}

pub trait Packager {
    const NAME: &'static str;

    type Error: Error + From<std::io::Error>;

    fn do_build_target(&self, target: &str, context: &Context) -> Result<(), Self::Error> {
        Command::new("cargo")
            .arg("build")
            .arg("--target")
            .arg(target)
            .current_dir(context.root)
            .status()?;
        Ok(())
    }

    fn do_build(&self, context: &Context) -> Result<(), Self::Error> {
        Ok(())
    }

    fn do_package_target(&self, target: &str, context: &Context) -> Result<(), Self::Error> {
        Ok(())
    }

    fn do_package(&self, context: &Context) -> Result<(), Self::Error> {
        Ok(())
    }
}
