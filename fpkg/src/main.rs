// Copyright (c) 2026, BlockProject 3D
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

use std::path::PathBuf;
use bp3d_util::result::ResultExt;
use clap::Parser;
use crate::args::{Args, Command};
use crate::project::Project;
use current_platform::CURRENT_PLATFORM;

mod args;
mod config;
mod project;
mod source;

fn main() {
    let mut args = Args::parse();
    if args.targets.is_empty() {
        args.targets.push(CURRENT_PLATFORM.into());
    }
    let mut project = Project::new(&args.root.unwrap_or(PathBuf::from("."))).expect_exit("unable to load project configuration", 1);
    if let Some(path) = bp3d_os::dirs::system::get_user_home() {
        project.add_config_if_exists(&path.join("fpkg.toml")).expect_exit("unable to load user supplied config", 1);
    }
    let exe = bp3d_os::assets::get_executable_path().unwrap();
    project.add_config_if_exists(&exe.join("../etc/fpkg.toml")).expect_exit("unable to load built-in config", 1);
    project.add_config_if_exists(&exe.join("../../res/config/fpkg.toml")).expect_exit("unable to load built-in config", 1);
    let params: Vec<String> = std::env::args().filter(|v| v.starts_with("FPKG_PARAM_")).map(|v| v[12..].to_lowercase()).collect();
    project.load_sources(&params).expect_exit("unable to load package sources", 1);
    match args.cmd {
        Command::Install => {
            for target in args.targets {
                project.install(&target).expect_exit("unable to install", 1);
            }
        },
        Command::Publish => {
            for target in args.targets {
                project.publish(&target).expect_exit("unable to publish", 1);
            }
        },
        Command::Clean => {
            for target in args.targets {
                project.clean(&target).expect_exit("unable to clean", 1);
            }
        }
    }
}
