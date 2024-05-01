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

mod packager;
mod manifest_ext;

use std::error::Error;
use std::path::PathBuf;
use cargo_toml::Manifest;
use clap::{command, Parser, ValueEnum};
use current_platform::CURRENT_PLATFORM;
use serde::de::DeserializeOwned;
use crate::manifest_ext::parse_manifest;
use crate::packager::{Config, Context, Framework, Packager};

#[derive(ValueEnum, Debug, Copy, Clone)]
pub enum PackageType {
    #[value(help = "A macOS/iOS framework bundle generator")]
    Framework
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 't', long = "target", help = "Specify which target(s) to build for.")]
    target_list: Vec<String>,

    #[arg(long, help = "Build rust targets in release mode.")]
    release: bool,

    #[arg(short = 'p', long = "package", required = true, help = "The packager engine to use.")]
    package_type: PackageType,

    #[arg(help = "Root path of the crate, where to find the manifest (Cargo.toml).")]
    root: Option<PathBuf>
}

trait ResultExt<T> {
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

fn run_packager<T: Packager + DeserializeOwned>(context: &Context) {
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

fn main() {
    let mut args = Args::parse();
    if args.target_list.len() == 0 {
        args.target_list.push(CURRENT_PLATFORM.into());
    }
    let collected: Vec<&str> = args.target_list.iter().map(|v| &**v).collect();
    let root = args.root.unwrap_or(PathBuf::from("./"));
    let ctx = Context {
        root: &root,
        manifest: Manifest::from_path(&root.join("Cargo.toml")).expect_exit("Failed to load root manifest"),
        config: if args.release { Config::Release } else { Config::Debug },
        targets: &collected
    };
    run_packager::<Framework>(&ctx);
}
