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

use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};
use bp3d_util::simple_error;
use bpx::core::Container;
use bpx::package::{Architecture, Package, Platform};
use bpx::package::util::unpack;
use bp3d_debug::debug;
use crate::config::{parse_config, parse_standalone_config, Config};
use crate::source::interface::{Dependency, Source};
use crate::source::registry::get_provider;

simple_error! {
    pub Error {
        Config(crate::config::Error) => "config error: {}",
        InvalidUrl(String) => "invalid URL: {:?}",
        UnknownScheme(String) => "unknown URL scheme: {}",
        Provider(crate::source::interface::Error) => "source configuration error: {}",
        Source(crate::source::interface::Error) => "source error: {}",
        Io(std::io::Error) => "io error: {}",
        UnknownSource(String) => "unknown source: {}",
        DependencyNotFound(Dependency) => "dependency {} not found",
        Bpxp(bpx::package::error::Error) => "bpxp error: {}",
        IncompatibleArch(Dependency) => "incompatible architecture for dependency: {}",
        IncompatiblePlatform(Dependency) => "incompatible platform for dependency: {}",
        Bpx(bpx::core::error::Error) => "bpx error: {}"
    }
}

fn get_platform_from_target(target: &str) -> Platform {
    if target.contains("windows") {
        Platform::Windows
    } else if target.contains("apple") {
        Platform::Mac
    } else if target.contains("linux") {
        Platform::Linux
    } else {
        Platform::Any
    }
}

fn get_architecture_from_target(target: &str) -> Architecture {
    if target.contains("x86_64") {
        Architecture::X86_64
    } else if target.contains("x86") {
        Architecture::X86
    } else if target.contains("aarch64") || target.contains("arm64") {
        Architecture::Aarch64
    } else if target.contains("arm") {
        Architecture::Armv7hl
    } else {
        Architecture::Any
    }
}

fn ensure_bpxp_compatible<T>(package: &Package<T>, dep: Dependency, target: &str) -> Result<(), Error> {
    let arch = get_architecture_from_target(target);
    let platform = get_platform_from_target(target);
    if arch != package.settings().architecture {
        return Err(Error::IncompatibleArch(dep))
    }
    if platform != package.settings().platform {
        return Err(Error::IncompatiblePlatform(dep))
    }
    Ok(())
}

fn publish_packages_rec(path: &Path, target: &str, source: &mut dyn Source) -> Result<(), Error> {
    debug!("Publishing packages in {:?}...", path);
    for entry in read_dir(path).map_err(Error::Io)? {
        let entry = entry.map_err(Error::Io)?;
        if entry.path().is_dir() {
            publish_packages_rec(&entry.path(), target, source)?;
        } else {
            if let Some(ext) = entry.path().extension() {
                if ext == "bpx" {
                    let file = File::open(&entry.path()).map_err(Error::Io)?;
                    let bpx = Container::open(file).map_err(Error::Bpx)?;
                    if bpx.main_header().ty != b'P' {
                        debug!("Skipping {:?}: not a BPXP file", entry.path());
                    }
                    let package = Package::try_from(bpx).map_err(Error::Bpxp)?;
                    let arch = get_architecture_from_target(target);
                    let platform = get_platform_from_target(target);
                    if arch != package.settings().architecture || platform != package.settings().platform {
                        debug!("Skipping {:?}: package is not compatible with published target", entry.path());
                        continue;
                    }
                    if let Some(val) = package.load_metadata().map_err(Error::Bpxp)?.as_object() {
                        let name = val.get("Name").map(|v| v.as_str()).flatten();
                        let version = val.get("Version").map(|v| v.as_str()).flatten();
                        match (name, version) {
                            (Some(name), Some(version)) => {
                                let dep = Dependency::new(name, version);
                                debug!({target}, "Publishing {:?} ({})...", entry.path(), dep);
                                source.ensure_valid_package(&dep).map_err(Error::Source)?;
                                source.publish(&dep, target, &entry.path()).map_err(Error::Source)?;
                            },
                            _ => {
                                debug!("Skipping {:?}: package is not a valid FPKG binary package", entry.path());
                                continue;
                            }
                        }
                    } else {
                        debug!("Skipping {:?}: package is not a valid FPKG binary package", entry.path());
                        continue;
                    }
                }
            } else {
                debug!("Skipping {:?}: not a BPX file", entry.path());
            }
        }
    }
    Ok(())
}

pub struct Project {
    config: Config,
    sources: HashMap<String, Box<dyn Source>>,
    path: PathBuf
}

impl Project {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let config = parse_config(path).map_err(Error::Config)?;
        Ok(Self {
            config: config.unwrap_or_default(),
            sources: HashMap::new(),
            path: PathBuf::from(path)
        })
    }

    pub fn add_config_if_exists(&mut self, path: &Path) -> Result<(), Error> {
        if !path.exists() || !path.is_file() {
            return Ok(());
        }
        debug!("adding config path: {:?}...", &path);
        let config = parse_standalone_config(path).map_err(Error::Config)?;
        if self.config.default_source.is_none() && config.default_source.is_some() {
            self.config.default_source = config.default_source;
        }
        self.config.sources.extend(config.sources.into_iter());
        Ok(())
    }

    pub fn load_sources(&mut self) -> Result<(), Error> {
        for (name, cfg) in &self.config.sources {
            let scheme = cfg.scheme().ok_or_else(|| Error::InvalidUrl(cfg.url.clone()))?;
            let provider = get_provider(scheme).ok_or_else(|| Error::UnknownScheme(scheme.into()))?;
            let source = provider.open(cfg.path(), &cfg.params).map_err(Error::Provider)?;
            self.sources.insert(name.into(), source);
        }
        Ok(())
    }

    pub fn install(&mut self, target: &str) -> Result<(), Error> {
        let target_path = self.path.join("target").join(target).join("ext");
        std::fs::create_dir_all(&target_path).map_err(Error::Io)?;
        for (name, dep) in &self.config.dependencies {
            let source = self.sources.get_mut(&dep.source).ok_or_else(|| Error::UnknownSource(dep.source.clone()))?;
            let mut dep = Dependency::new(name, &dep.version);
            if dep.version() == "latest" {
                dep = source.find_latest(name).map_err(Error::Source)?.ok_or_else(|| Error::DependencyNotFound(dep))?;
            }
            let dst_path = target_path.join(dep.get_package_filename());
            if dst_path.exists() && dst_path.is_file() {
                debug!("Not installing already existing dependency: {}", dep);
                continue;
            }
            debug!("Downloading dependency {}...", dep);
            source.download(&dep, target, &dst_path).map_err(Error::Source)?;
            debug!("Checking dependency {}...", dep);
            let file = File::open(&dst_path).map_err(Error::Io)?;
            let package = Package::open(&file).map_err(Error::Bpxp)?;
            ensure_bpxp_compatible(&package, dep.clone(), target)?;
            debug!("Unpacking dependency {}...", dep);
            unpack(&package, &target_path).map_err(Error::Bpxp)?;
            debug!("Installed dependency {}!", dep);
        }
        Ok(())
    }

    pub fn publish(&mut self, target: &str) -> Result<(), Error> {
        let pubsrc = self.config.default_source.as_deref().unwrap_or("default");
        let source = self.sources.get_mut(pubsrc).ok_or_else(|| Error::UnknownSource(pubsrc.into()))?;
        let target_path = self.path.join("target").join(target);
        publish_packages_rec(&target_path.join("debug"), target, &mut **source)?;
        publish_packages_rec(&target_path.join("release"), target, &mut **source)
    }
}
