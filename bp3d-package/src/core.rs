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

use std::collections::VecDeque;
use bp3d_util::result::ResultExt;
use crate::manifest_ext::parse_manifest;
use crate::packager::{Context, Packager};

pub fn run_packager<'a, T: Packager<'a>>(context: &'a Context) {
    println!("Initializing packager {}...", T::NAME);
    let config: Option<T::Config> = parse_manifest(context.path, context.packager)
        .expect_exit("Failed to load packager configuration from root manifest", 1);
    let packager = T::new(config, context).expect_exit("Failed to initialize packager", 1);
    println!("Building targets...");
    let mut v = VecDeque::new();
    for target in context.targets {
        println!("Building target '{}'...", target);
        let data = packager.do_build_target(target).expect_exit("Failed to build target", 1);
        v.push_back(data);
    }
    println!("Running post build phase...");
    packager.do_build().expect_exit("Failed to run post-build phase", 1);
    println!("Packaging targets...");
    for target in context.targets {
        println!("Packaging target '{}'...", target);
        let data = v.pop_front().unwrap();
        packager.do_package_target(&data, target).expect_exit("Failed to package target", 1);
    }
    println!("Generating full package...");
    packager.do_package().expect_exit("Failed to generate full package", 1);
}
