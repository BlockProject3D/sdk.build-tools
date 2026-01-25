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
use bp3d_util::simple_error;
use crate::build::cargo::{CargoBuilder, CargoWorkspace};
use crate::build::lua::{LuaBuilder, LuaPackage};
use crate::system::{BuildSystem, Context, Features, Package};
use crate::system::artifact::List;

simple_error! {
    pub Error {
        UnknownProject => "unknown project configuration",
        InvalidPackage(String) => "invalid package: {}",
        InvalidTarget(String) => "invalid target: {}",
        InvalidConfig(String) => "invalid configuration: {}",
        UnknownFeature(String) => "unknown feature: {}",
        BuildSystem(String) => "build system: {}"
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait BuildTool {
    fn package(&self) -> &dyn Package;

    fn configure(&self, ctx: &Context, targets: &[&str]) -> Result<()>;

    fn build(&self, ctx: &Context, target: &str) -> Result<()>;

    fn pre_package(&self, ctx: &Context, target: &str) -> Result<List>;
}

struct BuildSystemWrapper<P, B> {
    package: P,
    build_system: B
}

impl<P, B> BuildSystemWrapper<P, B> {
    fn new(package: P, build_system: B) -> Self {
        Self { package, build_system }
    }
}

impl<P, B> BuildSystemWrapper<P, B>
    where P: Package {
    fn check_context(&self, ctx: &Context, target: &str) -> Result<()> {
        let targets = self.package.targets();
        let features = self.package.features();
        let configurations = self.package.configurations();
        let found = targets.iter().any(|v| v == target);
        if !found {
            return Err(Error::InvalidTarget(target.into()));
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
        Ok(())
    }
}

impl<P, B> BuildTool for BuildSystemWrapper<P, B>
    where P: Package, B: BuildSystem<Package = P> {
    fn package(&self) -> &dyn Package {
        &self.package
    }

    fn configure(&self, ctx: &Context, targets: &[&str]) -> Result<()> {
        for v in targets {
            self.check_context(ctx, v)?;
        }
        self.build_system.configure(&self.package, ctx, targets).map_err(|v| Error::BuildSystem(v.to_string()))
    }

    fn build(&self, ctx: &Context, target: &str) -> Result<()> {
        self.check_context(ctx, target)?;
        self.build_system.build(&self.package, &ctx, target).map_err(|v| Error::BuildSystem(v.to_string()))
    }

    fn pre_package(&self, ctx: &Context, target: &str) -> Result<List> {
        self.check_context(ctx, target)?;
        self.build_system.pre_package(&self.package, &ctx, target).map_err(|v| Error::BuildSystem(v.to_string()))
    }
}

pub fn open(path: &Path) -> Result<Box<dyn BuildTool>> {
    let manifest = path.join("Cargo.toml");
    if manifest.exists() {
        let package = CargoWorkspace::load(path).map_err(|e| Error::InvalidPackage(e.to_string()))?;
        Ok(Box::new(BuildSystemWrapper::new(package, CargoBuilder)))
    } else if path.join("build.lua").exists() {
        let package = LuaPackage::new(path).map_err(|e| Error::InvalidPackage(e.to_string()))?;
        Ok(Box::new(BuildSystemWrapper::new(package, LuaBuilder)))
    } else {
        Err(Error::UnknownProject)
    }
}
