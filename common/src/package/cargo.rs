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

use std::path::Path;
use std::process::Command;
use cargo_toml::Manifest;
use crate::output::Output;
use super::Package;
use super::Error;
use super::Context;

pub struct Cargo {
    manifest: Manifest,
    outputs: Vec<Output<'static>>
}

impl Cargo {
    pub fn load<'a>(root: &'a Path, config: &'a str) -> Result<Option<Context<'a>>, Error> {
        let path = root.join("Cargo.toml");
        if !path.exists() {
            return Ok(None);
        }
        if config != "debug" && config != "release" {
            return Err(Error::InvalidConfig(config.into()));
        }
        let manifest = Manifest::from_path(path).map_err(Error::Cargo)?;
        let outputs = manifest.bin.iter().map(|v| Output::Bin(v.name.clone()
            .unwrap_or(manifest.package().name().into()).into()))
            .chain(
                manifest.lib.iter()
                    .map(|v| Output::Lib(v.name.clone().unwrap_or(manifest.package().name().into()).into()))
            ).collect();
        let cargo = Cargo {
            manifest,
            outputs
        };
        Ok(Some(Context { root, package: Box::new(cargo), config }))
    }
}

const SUPPORTED_TARGETS: &[&str] = &[
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "aarch64-apple-ios",
    "aarch64-unknown-linux-gnu",
    "x86_64-unknown-linux-gnu",
    "aarch64-pc-windows-msvc",
    "x86_64-pc-windows-msvc"
];

impl Package for Cargo {
    fn get_name(&self) -> &str {
        self.manifest.package().name()
    }

    fn get_version(&self) -> &str {
        self.manifest.package().version()
    }

    fn get_outputs(&self) -> &[Output] {
        &self.outputs
    }

    fn pre_build(&self, ctx: &Context, target: &str) -> Result<(), Error> {
        if ctx.config == "release" {
            Command::new("cargo")
                .arg("build")
                .arg("--release")
                .arg("--target")
                .arg(target)
                .current_dir(ctx.root)
                .status().map_err(Error::Io)?;
        } else {
            Command::new("cargo")
                .arg("build")
                .arg("--target")
                .arg(target)
                .current_dir(ctx.root)
                .status().map_err(Error::Io)?;
        }
        Ok(())
    }

    fn is_valid_target(&self, target: &str) -> bool {
        SUPPORTED_TARGETS.contains(&target)
    }
}
