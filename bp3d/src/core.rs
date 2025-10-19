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

use bp3d_debug::{debug, info};
use bp3d_util::result::ResultExt;
use bp3d_build::core;
use bp3d_build::system::Context;
use crate::args::Command;

fn run_command(tool: &dyn core::BuildTool, ctx: Context, cmd: Command) -> core::Result<()> {
    debug!("Running command: {:?} for package {}-{}", cmd, tool.package().get_name(), tool.package().get_version());
    match cmd {
        Command::Configure => {
            info!("Configuring package...");
            tool.configure(&ctx)
        },
        Command::Build => {
            info!("Configuring package...");
            tool.configure(&ctx)?;
            info!("Building package...");
            tool.build(&ctx)
        }
        Command::PrePackage => {
            info!("Configuring package...");
            tool.configure(&ctx)?;
            info!("Pre-packaging package...");
            tool.pre_package(&ctx).map(|_| ())
        }
        Command::Package => {
            info!("Configuring package...");
            tool.configure(&ctx)?;
            todo!()
        }
    }
}

pub fn dispatch_run(ctx: Context, cmd: Command) {
    let tool = core::open(&ctx).expect_exit("Failed to load package", 1);
    run_command(&*tool, ctx, cmd).expect_exit("Failed to run build", 2);
}
