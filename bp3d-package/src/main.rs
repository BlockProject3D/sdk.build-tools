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
mod core;
mod args;

use std::path::PathBuf;
use cargo_toml::Manifest;
use clap::Parser;
use current_platform::CURRENT_PLATFORM;
use crate::args::Args;
use crate::core::ResultExt;
use crate::packager::interface::{Config, Context};

fn main() {
    let mut args = Args::parse();
    if args.target_list.len() == 0 {
        args.target_list.push(CURRENT_PLATFORM.into());
    }
    let collected: Vec<&str> = args.target_list.iter().map(|v| &**v).collect();
    let root = args.root.unwrap_or(PathBuf::from("./"));
    let ctx = Context {
        root: &root,
        package: Manifest::from_path(&root.join("Cargo.toml")).expect_exit("Failed to load root manifest"),
        config: if args.release { Config::Release } else { Config::Debug },
        targets: &collected
    };
    args.package_type.call(&ctx);
}
