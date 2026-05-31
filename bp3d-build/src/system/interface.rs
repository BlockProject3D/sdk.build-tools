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

use std::borrow::Cow;
use std::ops::Deref;
use std::path::Path;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Features<'a> {
    All,
    List(&'a [&'a str])
}

impl<'a> Deref for Features<'a> {
    type Target = [&'a str];

    fn deref(&self) -> &Self::Target {
        match self {
            Features::All => &[],
            Features::List(v) => v
        }
    }
}

/*pub struct Context<'a> {
    pub path: &'a Path,
    pub target: &'a str,
    pub configuration: &'a str,
    pub features: Features<'a>
}*/

pub struct Context<'a> {
    pub path: &'a Path,
    pub configuration: &'a str,
    pub features: Features<'a>
}

pub trait BuildSystem {
    type Error: std::error::Error;
    type Package: Package;

    /// Configure the build.
    fn configure(&self, package: &Self::Package, ctx: &Context, targets: &[&str]) -> Result<(), Self::Error>;

    /// Build the project.
    fn build(&self, package: &Self::Package, ctx: &Context, target: &str) -> Result<(), Self::Error>;

    /// Prepares the project for packaging to a specific target.
    ///
    /// This function is intended to build a flat list of artifacts which can be used by
    /// bp3d-package.
    fn pre_package(&self, package: &Self::Package, ctx: &Context, target: &str) -> Result<crate::system::artifact::List, Self::Error>;
}

pub trait Component {
    fn get_name(&self) -> &str;

    fn get_version(&self) -> &str;

    fn get_short_name(&self) -> &str;

    fn get_description(&self) -> Option<&str>;
}

pub trait Package {
    /// Returns the name of the package.
    fn get_primary_name(&self) -> &str;

    /// Returns the version of this package.
    fn get_primary_version(&self) -> &str;

    /// Returns the number of sub packages.
    fn get_components(&self) -> usize;

    /// Returns a sub package by index.
    fn get_component(&self, index: usize) -> &dyn Component;

    /// Returns the list of available targets.
    fn targets(&self) -> &[Cow<'_, str>];

    /// Returns the list of available configurations.
    fn configurations(&self) -> &[Cow<'_, str>];

    /// Returns the list of available features.
    fn features(&self) -> &[Cow<'_, str>];
}
