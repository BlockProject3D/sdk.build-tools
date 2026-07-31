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

use crate::args::Args;
use crate::core::{Context, dispatch_run};
use bp3d_build::system::Features;
use bp3d_os::module::loader::ModuleLoader;
use clap::Parser;
use current_platform::CURRENT_PLATFORM;
use std::path::Path;

mod args;
mod core;

fn main() {
    let mut args = Args::parse();
    if args.targets.is_empty() {
        args.targets.push(CURRENT_PLATFORM.into());
    }
    let features: Vec<&str> = args.features.iter().map(|v| &**v).collect();
    let targets: Vec<&str> = args.targets.iter().map(|v| &**v).collect();
    let ctx = Context {
        path: args.root.as_deref().unwrap_or(Path::new("./")),
        targets: &targets,
        configuration: args.configuration.as_deref().unwrap_or("debug"),
        features: if args.all_features.unwrap_or(true) { // By default enable all features
            Features::All
        } else {
            Features::List(&features)
        },
    };
    ModuleLoader::install(&[]);
    dispatch_run(ctx, args.cmd, args.package_type, args.other_args);
    ModuleLoader::uninstall();
}
