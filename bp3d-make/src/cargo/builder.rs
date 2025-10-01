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
use crate::system::{BuildSystem, Context, Features};
use crate::system::artifact::{Artifact, LibType, List};
use super::Error;

pub struct CargoBuilder;

fn list_headers(path: &Path, name: &str, list: &mut List) -> Result<(), Error> {
    if path.exists() {
        let files = std::fs::read_dir(path).map_err(Error::Io)?;
        for file in files {
            let file = file.map_err(Error::Io)?;
            let ty = file.file_type().map_err(Error::Io)?;
            let name1 = String::from(name) + file.file_name().to_str().ok_or(Error::InvalidUtf8)?;
            if ty.is_file() {
                let artifact = Artifact::header(&file.path(), &name1);
                list.add(artifact);
            } else if ty.is_dir() {
                list_headers(&file.path(), &name1, list)?;
            }
        }
    }
    Ok(())
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
        cmd.arg("build").arg("--target").arg(ctx.target).current_dir(ctx.path);
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
        let include_folder = ctx.path.join("include");
        list_headers(&include_folder, "", &mut artifacts)?;
        Ok(artifacts)
    }
}
