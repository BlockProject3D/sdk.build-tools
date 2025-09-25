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

use std::path::{Path, PathBuf};
use bp3d_util::simple_error;
use crate::output::Output;

pub trait Package {
    /// Returns the name of the package.
    fn get_name(&self) -> &str;

    /// Returns the version of this package.
    fn get_version(&self) -> &str;

    /// Returns an iterator over all outputs of this package.
    fn get_outputs(&self) -> &[Output];

    /// Pre-builds the package for the specified target and context combinations.
    fn pre_build(&self, ctx: &Context, target: &str) -> Result<(), Error>;

    /// Returns true if the given target triple is known to the package type or not.
    fn is_valid_target(&self, target: &str) -> bool;
}

simple_error! {
    pub Error {
        Cargo(cargo_toml::Error) => "cargo manifest error: {}",
        InvalidConfig(String) => "invalid configuration name: {}",
        InvalidTarget(String) => "invalid target name: {}",
        Io(std::io::Error) => "io error: {}",
        UnknownPackage => "unknown package type"
    }
}

pub struct Context<'a> {
    pub root: &'a Path,
    pub package: Box<dyn Package>,
    pub config: &'a str
}

impl<'a> Context<'a> {
    pub fn get_target_path(&self, target: &str) -> PathBuf {
        self.root.join("target").join(target).join(self.config)
    }
}
