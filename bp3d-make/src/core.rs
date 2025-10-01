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
use bp3d_util::simple_error;
use crate::args::Command;
use crate::cargo::{CargoBuilder, CargoWorkspace};
use crate::system::{BuildSystem, Context, Features, Package};

simple_error! {
    pub Error {
        InvalidTarget(String) => "invalid target: {}",
        InvalidConfig(String) => "invalid configuration: {}",
        UnknownFeature(String) => "unknown feature: {}",
        BuildSystem(String) => "build system: {}",
        Json(serde_json::Error) => "json: {}"
    }
}

fn run_command<P: Package, B: BuildSystem<Package = P>>(ctx: Context, cmd: Command, package: P, build_system: B) -> Result<Option<String>, Error> {
    let targets = package.targets();
    let features = package.features();
    let configurations = package.configurations();
    let target = targets.iter().any(|v| v == ctx.target);
    if !target {
        return Err(Error::InvalidTarget(ctx.target.into()));
    }
    let configuration = configurations.iter().any(|v| v == ctx.configuration);
    if !configuration {
        return Err(Error::InvalidConfig(ctx.configuration.into()));
    }
    if let Features::List(list) = &ctx.features {
        for feature in *list {
            let exists = features.iter().any(|v| v == feature);
            if !exists {
                return Err(Error::UnknownFeature((*feature).into()));
            }
        }
    }
    debug!("Running command: {:?} for package {}-{}", cmd, package.get_name(), package.get_version());
    match cmd {
        Command::Configure => {
            info!("Configuring package...");
            build_system.configure(&package, &ctx).map_err(|v| Error::BuildSystem(v.to_string()))?;
        },
        Command::Build => {
            info!("Configuring package...");
            build_system.configure(&package, &ctx).map_err(|v| Error::BuildSystem(v.to_string()))?;
            info!("Building package...");
            build_system.build(&package, &ctx).map_err(|v| Error::BuildSystem(v.to_string()))?;
        }
        Command::PrePackage => {
            info!("Configuring package...");
            build_system.configure(&package, &ctx).map_err(|v| Error::BuildSystem(v.to_string()))?;
            info!("Pre-packaging package...");
            let list = build_system.pre_package(&package, &ctx).map_err(|v| Error::BuildSystem(v.to_string()))?;
            let output = serde_json::to_string(&list.into_inner()).map_err(Error::Json)?;
            return Ok(Some(output));
        }
    }
    Ok(None)
}

pub fn dispatch_run(ctx: Context, cmd: Command) -> Option<String> {
    let manifest = ctx.path.join("Cargo.toml");
    if manifest.exists() {
        let package = CargoWorkspace::load(ctx.path).expect_exit("Failed to load cargo package manifest", 1);
        run_command(ctx, cmd, package, CargoBuilder).expect_exit("Failed to run build", 2)
    } else {
        None
    }
}
