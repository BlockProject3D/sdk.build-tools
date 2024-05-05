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

use std::error::Error;
use cargo_toml::Manifest;
use serde::de::DeserializeOwned;
use crate::manifest_ext::parse_manifest;
use crate::packager::interface::{Context, Output, Package, Packager};

pub trait ResultExt<T> {
    fn expect_exit(self, msg: &str) -> T;
}

impl<T, E: Error> ResultExt<T> for Result<T, E> {
    fn expect_exit(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {}", msg, e);
                std::process::exit(1);
            }
        }
    }
}

impl Package for Manifest {
    fn get_name(&self) -> &str {
        self.package().name()
    }

    fn get_version(&self) -> &str {
        self.package().version()
    }

    fn get_outputs(&self) -> impl Iterator<Item = Output> {
        self.bin.iter().map(|v| Output::Bin(v.name.as_deref().unwrap_or(self.get_name()))).chain(self.lib.iter().map(|v| Output::Lib(v.name.as_deref().unwrap_or(self.get_name()))))
    }
}

pub fn run_packager<P: Package, T: Packager + DeserializeOwned>(context: &Context<P>) {
    println!("Initializing packager {}...", T::NAME);
    let packager: T = parse_manifest(context.root)
        .expect_exit("Failed to load packager configuration from root manifest");
    println!("Building targets...");
    for target in context.targets {
        println!("Building target '{}'...", target);
        packager.do_build_target(target, context).expect_exit("Failed to build target");
    }
    println!("Running post build phase...");
    packager.do_build(context).expect_exit("Failed to run post-build phase");
    println!("Packaging targets...");
    for target in context.targets {
        println!("Packaging target '{}'...", target);
        packager.do_package_target(target, context).expect_exit("Failed to package target");
    }
    println!("Generating full package...");
    packager.do_package(context).expect_exit("Failed to generate full package");
}
