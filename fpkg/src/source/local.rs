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

use crate::config::Parameters;
use crate::source::interface::{Dependency, Error, Provider, Result, Source};
use bp3d_util::path::PathExt;
use std::path::{Path, PathBuf};

const DEFAULT_MAX_DEP_NAME_SIZE: i64 = 32;

struct Local {
    path: PathBuf,
    dep_name_size_limit: usize,
}

impl Source for Local {
    fn ensure_valid_package(&mut self, dep: &Dependency) -> Result<()> {
        if dep.name().len() > self.dep_name_size_limit {
            return Err(Error::from("too many bytes in dependency name"));
        }
        Ok(())
    }

    fn publish(&mut self, dep: &Dependency, target: &str, src_file: &Path) -> Result<()> {
        let path = self.path.join(dep.name()).join(dep.version());
        std::fs::create_dir_all(&path).map_err(Error::Io)?;
        let useless = path.join(target);
        let target = useless.ensure_extension("bpx");
        std::fs::copy(src_file, target).map_err(Error::Io)?;
        Ok(())
    }

    fn find_latest(&mut self, name: &str) -> Result<Option<Dependency>> {
        let path = self.path.join(name);
        let mut version = None;
        for res in std::fs::read_dir(&path).map_err(Error::Io)? {
            let entry = res.map_err(Error::Io)?;
            let name = version.get_or_insert(entry.file_name());
            if &entry.file_name() > name {
                version = Some(entry.file_name());
            }
        }
        if let Some(version) = version {
            let version = version
                .to_str()
                .ok_or_else(|| Error::InvalidDep(Dependency::new(name, "latest")))?;
            let target = path.join(version);
            if target.exists() && target.is_dir() {
                Ok(Some(Dependency::new(name, version)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn find(&mut self, name: &str, version: &str) -> Result<Option<Dependency>> {
        if let Some(dep) = self.find_latest(name)? {
            return Ok(Some(dep));
        }
        let path = self.path.join(name).join(version);
        if path.exists() && path.is_dir() {
            Ok(Some(Dependency::new(name, version)))
        } else {
            Ok(None)
        }
    }

    fn download(&mut self, dep: &Dependency, target: &str, target_path: &Path) -> Result<()> {
        let path = self
            .path
            .join(dep.name())
            .join(dep.version())
            .join(format!("{}.bpx", target));
        if path.exists() && path.is_file() {
            std::fs::copy(path, target_path).map_err(Error::Io)?;
            Ok(())
        } else {
            Err(Error::MissingDep(dep.clone()))
        }
    }
}

struct LocalProvider;

impl Provider for LocalProvider {
    fn open(&self, path: &str, params: &Parameters) -> Result<Box<dyn Source>> {
        let dep_name_max_size = params
            .get("max-dep-name-size")
            .map(|v| v.as_integer())
            .unwrap_or(Some(DEFAULT_MAX_DEP_NAME_SIZE))
            .ok_or(Error::InvalidParameter("max-dep-name-size"))?;
        Ok(Box::new(Local {
            path: path.into(),
            dep_name_size_limit: dep_name_max_size as _,
        }))
    }

    fn scheme(&self) -> &str {
        "local"
    }
}

pub static LOCAL: &dyn Provider = &LocalProvider;
