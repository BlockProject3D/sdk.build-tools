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

use std::borrow::Cow;
use std::path::Path;
use cargo_toml::Manifest;
use crate::system::{static_string, Package};
use super::Error;

const SUPPORTED_TARGETS: &[Cow<str>] = &[
    static_string("aarch64-apple-darwin"),
    static_string("x86_64-apple-darwin"),
    static_string("aarch64-apple-ios"),
    static_string("aarch64-unknown-linux-gnu"),
    static_string("x86_64-unknown-linux-gnu"),
    static_string("aarch64-pc-windows-msvc"),
    static_string("x86_64-pc-windows-msvc")
];

const SUPPORTED_CONFIGURATIONS: &[Cow<str>] = &[
    static_string("debug"),
    static_string("release")
];

pub struct CargoWorkspace {
    packages: Vec<CargoPackage>,
    core_name: String,
    core_version: String,
    features: Vec<Cow<'static, str>>
}

impl CargoWorkspace {
    pub fn load(root: &Path) -> Result<CargoWorkspace, Error> {
        let manifest = Manifest::from_path(root.join("Cargo.toml")).map_err(Error::Cargo)?;
        let mut packages = Vec::new();
        let mut core_name: Option<String> = None;
        let mut core_version: Option<String> = None;
        match manifest.workspace {
            Some(v) => {
                for member in v.members {
                    let package = CargoPackage::load(&root.join(&member).join("Cargo.toml"))?;
                    if core_name.is_none() {
                        core_name = Some(package.get_name().into());
                        core_version = Some(package.get_version().into());
                    }
                    if member == "core" {
                        core_name = Some(package.get_name().into());
                        core_version = Some(package.get_version().into());
                    }
                    packages.push(package);
                }
            },
            None => {
                let package = CargoPackage::open(manifest);
                core_name = Some(package.get_name().into());
                core_version = Some(package.get_version().into());
                packages.push(package)
            }
        }
        let features = packages.iter().map(|v| v.features().iter().map(|v| String::from(&**v).into())).flatten().collect();
        Ok(CargoWorkspace { packages, core_name: core_name.unwrap(), core_version: core_version.unwrap(), features })
    }

    pub fn bins(&self) -> impl Iterator<Item = &str> {
        self.packages.iter().map(|v| v.bins()).flatten()
    }

    pub fn libs(&self) -> impl Iterator<Item = &str> {
        self.packages.iter().map(|v| v.libs()).flatten()
    }
}

impl Package for CargoWorkspace {
    fn get_name(&self) -> &str {
        &self.core_name
    }

    fn get_version(&self) -> &str {
        &self.core_version
    }

    fn targets(&self) -> &[Cow<str>] {
        SUPPORTED_TARGETS
    }

    fn configurations(&self) -> &[Cow<str>] {
        SUPPORTED_CONFIGURATIONS
    }

    fn features(&self) -> &[Cow<str>] {
        &self.features
    }
}

struct CargoPackage {
    manifest: Manifest,
    features: Vec<Cow<'static, str>>
}

impl CargoPackage {
    pub fn open(manifest: Manifest) -> CargoPackage {
        let features = manifest.features.iter().map(|(name, _)| name.clone().into()).collect();
        CargoPackage {
            manifest,
            features
        }
    }

    pub fn load(path: &Path) -> Result<CargoPackage, Error> {
        let manifest = Manifest::from_path(path).map_err(Error::Cargo)?;
        Ok(Self::open(manifest))
    }

    pub fn bins(&self) -> impl Iterator<Item = &str> {
        self.manifest.bin.iter().map(|v| v.name.as_deref().unwrap_or(self.manifest.package().name()))
    }

    pub fn libs(&self) -> impl Iterator<Item = &str> {
        self.manifest.lib.iter().map(|v| v.name.as_deref().unwrap_or(self.manifest.package().name()))
    }
}

impl Package for CargoPackage {
    fn get_name(&self) -> &str {
        self.manifest.package().name()
    }

    fn get_version(&self) -> &str {
        self.manifest.package().version()
    }

    fn targets(&self) -> &[Cow<str>] {
        SUPPORTED_TARGETS
    }

    fn configurations(&self) -> &[Cow<str>] {
        SUPPORTED_CONFIGURATIONS
    }

    fn features(&self) -> &[Cow<str>] {
        self.features.as_slice()
    }
}
