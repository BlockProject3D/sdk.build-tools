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

use std::path::Path;
use bp3d_debug::{debug, info};
use bp3d_util::result::ResultExt;
use bp3d_build::core;
use bp3d_build::system::Features;
use bp3d_package::packager::lua::Lua;
use bp3d_package::packager::PackagerType;
use bp3d_package::run_packager;
use crate::args::Command;

pub struct Context<'a> {
    pub path: &'a Path,
    pub configuration: &'a str,
    pub targets: &'a [&'a str],
    pub features: Features<'a>
}

fn run_command(tool: &dyn core::BuildTool, ctx: Context, cmd: Command, packager: Option<String>) -> core::Result<()> {
    debug!("Running command: {:?} for package {}-{}", cmd, tool.package().get_primary_name(), tool.package().get_primary_version());
    let ctx2 = bp3d_build::system::Context {
        path: ctx.path,
        configuration: ctx.configuration,
        features: ctx.features
    };
    match cmd {
        Command::Configure => {
            info!("Configuring package for targets {:?}...", ctx.targets);
            tool.configure(&ctx2, ctx.targets)
        },
        Command::Build => {
            info!("Configuring package for targets {:?}...", ctx.targets);
            tool.configure(&ctx2, ctx.targets)?;
            for target in ctx.targets {
                info!("Building package for target {}...", target);
                tool.build(&ctx2, target)?;
            }
            Ok(())
        }
        Command::PrePackage => {
            info!("Configuring package for targets {:?}...", ctx.targets);
            tool.configure(&ctx2, ctx.targets)?;
            for target in ctx.targets {
                info!("Building package for target {}...", target);
                tool.pre_package(&ctx2, target)?;
            }
            Ok(())
        }
        Command::Package => {
            if let Some(packager_name) = packager {
                let ctx = bp3d_package::packager::Context {
                    path: ctx.path,
                    configuration: ctx.configuration,
                    targets: ctx.targets,
                    tool,
                    packager: &packager_name
                };
                let packager = PackagerType::from_name(&packager_name);
                match packager {
                    Some(packager) => packager.call(&ctx),
                    None => run_packager::<Lua>(&ctx)
                }
                Ok(())
            } else {
                eprintln!("Please specify a packager type to run the packaging process");
                std::process::exit(1);
            }
        }
    }
}

pub fn dispatch_run(ctx: Context, cmd: Command, packager: Option<String>) {
    let tool = core::open(&ctx.path).expect_exit("Failed to load package", 1);
    run_command(&*tool, ctx, cmd, packager).expect_exit("Failed to run build", 2);
}
