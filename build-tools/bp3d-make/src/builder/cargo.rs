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

use cargo_toml::Manifest;
use crate::builder::interface::{Builder, Context, Module, OutputList};
use std::fmt::Display;
use std::fmt::Formatter;
use std::process::Command;
use bp3d_sdk_util::simple_error;
use crate::builder::util::PathExt;

simple_error! {
    Error {
        Io(std::io::Error) => "io error: {}",
        Cargo(cargo_toml::Error) => "cargo manifest error: {}"
    }
}

pub struct Cargo {
    manifests: Vec<Manifest>
}

fn list_outputs<'a>(manifest: &'a Manifest, context: &Context, module: &Module, outputs: &mut OutputList<'a>) -> Result<(), Error> {
    let base_path = module.path.join("target").join_option(context.target)
        .join(if context.release { "release" } else { "debug" });
    outputs.add_target_path(base_path);
    for bin in &manifest.bin {
        //let bin_name = String::from(bin.name.as_deref().unwrap_or(manifest.package().name())) + context.get_exe_extension();
        //let path = base_path.join(bin_name);
        //outputs.add_bin(path.into());
        outputs.add_bin(bin.name.as_deref().unwrap_or(manifest.package().name()));
    }
    if let Some(lib) = &manifest.lib {
        //let mut bin_name = None;
        outputs.add_lib(lib.name.as_deref().unwrap_or(manifest.package().name()));
        /*if lib.crate_type.contains(&"staticlib".into()) {
            bin_name = Some(String::from(lib.name.as_deref().unwrap_or(manifest.package().name())) + context.get_staticlib_extension());
        } else if lib.crate_type.contains(&"cdylib".into()) || lib.crate_type.contains(&"dylib".into()) {
            bin_name = Some(String::from(lib.name.as_deref().unwrap_or(manifest.package().name())) + context.get_dynlib_extension());
        }*/
        /*if let Some(bin_name) = bin_name {
            let path = base_path.join(bin_name);
            outputs.add_lib(path.into());
        }*/
    }
    Ok(())
}

impl Builder for Cargo {
    const NAME: &'static str = "Cargo";
    type Error = Error;

    fn do_configure(_: &Context, module: &Module) -> Result<Self, Self::Error> {
        let manifest = Manifest::from_path(module.path.join("Cargo.toml"))
            .map_err(Error::Cargo)?;
        let mut manifests = Vec::new();
        if let Some(workspace) = &manifest.workspace {
            for member in &workspace.members {
                let manifest = Manifest::from_path(module.path.join(member).join("Cargo.toml"))
                    .map_err(Error::Cargo)?;
                manifests.push(manifest);
            }
        }
        manifests.push(manifest);
        Ok(Cargo {
            manifests
        })
    }

    fn do_compile(&self, context: &Context, module: &Module) -> Result<(), Self::Error> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build");
        if context.release {
            cmd.arg("--release");
        }
        if context.all_features {
            cmd.arg("--all-features");
        } else if context.features.len() > 0 {
            cmd.arg("--features").args(context.features);
        }
        if let Some(target) = context.target {
            cmd.arg("--target").arg(target);
        }
        cmd.current_dir(module.path)
            .status()
            .map_err(Error::Io)?;
        Ok(())
    }

    fn list_outputs(&self, context: &Context, module: &Module) -> Result<OutputList, Self::Error> {
        let mut outputs = OutputList::new();
        for manifest in &self.manifests {
            list_outputs(manifest, context, module, &mut outputs)?;
        }
        Ok(outputs)
    }
}
