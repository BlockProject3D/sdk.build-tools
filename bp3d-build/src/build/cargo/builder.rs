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

use std::process::Command;
use crate::system::{BuildSystem, Context, Features, Package};
use crate::system::artifact::{Artifact, LibType, List, Type};
use super::Error;

pub struct CargoBuilder;

fn gen_base_command(cmd: &mut Command, ctx: &Context) {
    cmd.arg("--target").arg(ctx.target).current_dir(ctx.path);
    if ctx.configuration == "release" {
        cmd.arg("--release");
    }
    if ctx.features == Features::All {
        cmd.arg("--all-features");
    } else if ctx.features.len() > 0 {
        cmd.arg("--features");
        for v in ctx.features.iter() {
            cmd.arg(v);
        }
    }
}

impl BuildSystem for CargoBuilder {
    type Error = Error;
    type Package = super::CargoWorkspace;

    fn configure(&self, _: &Self::Package, _: &Context) -> Result<(), Self::Error> {
        Ok(())
    }

    fn build(&self, _: &Self::Package, ctx: &Context) -> Result<(), Self::Error> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build").current_dir(ctx.path);
        if ctx.configuration == "release" {
            cmd.arg("--release");
        }
        if ctx.features == Features::All {
            cmd.arg("--all-features");
        } else if ctx.features.len() > 0 {
            cmd.arg("--features");
            for v in ctx.features.iter() {
                cmd.arg(v);
            }
        }
        cmd.status().map_err(Error::Io)?;
        Ok(())
    }

    fn pre_package(&self, package: &Self::Package, ctx: &Context) -> Result<List, Self::Error> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build");
        gen_base_command(&mut cmd, ctx);
        cmd.status().map_err(Error::Io)?;
        let mut artifacts = List::new();
        let target_folder = ctx.path.join("target").join(ctx.target).join(ctx.configuration);
        for lib in package.libs() {
            let dy = Artifact::find_lib(&target_folder, lib, LibType::Dynamic);
            let st = Artifact::find_lib(&target_folder, lib, LibType::Static);
            artifacts.add_if_some(dy);
            artifacts.add_if_some(st);
        }
        for bin in package.bins() {
            let bin = Artifact::find_bin(&target_folder, bin);
            artifacts.add_if_some(bin);
        }
        if artifacts.find(Type::Lib(LibType::Dynamic)).count() > 0 && ctx.target.contains("apple") {
            println!("Adding version information...");
            let version = package.get_version();
            let mut cmd = Command::new("cargo");
            cmd.arg("rustc");
            gen_base_command(&mut cmd, ctx);
            cmd.arg("--").arg(format!("-Clink-arg=-compatibility_version{}", version))
                .arg(format!("-Clink-arg=-current_version{}", version));
            cmd.status().map_err(Error::Io)?;
        } else if ctx.target.contains("msvc") {
            // We have a windows build, include the RC file.

        }
        let mut artifacts = List::new();
        let target_folder = ctx.path.join("target").join(ctx.target).join(ctx.configuration);
        for lib in package.libs() {
            let dy = Artifact::find_lib(&target_folder, lib, LibType::Dynamic);
            let st = Artifact::find_lib(&target_folder, lib, LibType::Static);
            artifacts.add_if_some(dy);
            artifacts.add_if_some(st);
        }
        for bin in package.bins() {
            let bin = Artifact::find_bin(&target_folder, bin);
            artifacts.add_if_some(bin);
        }
        artifacts.add_folder(Type::Header, &ctx.path.join("include"), "").map_err(Error::Io)?;
        artifacts.add_folder(Type::Resource, &ctx.path.join("res"), "").map_err(Error::Io)?;
        Ok(artifacts)
    }
}
