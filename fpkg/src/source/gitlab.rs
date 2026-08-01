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

use super::interface::{Dependency, Error, Provider, Result, Source};
use crate::config::Parameters;
use bp3d_debug::error;
use glgp::util::{get_base_url, get_project_id};
use regex::Regex;
use std::boxed::Box;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::string::String;
use glgp::types::TokenType;

struct GitLab {
    list: glgp::list::PackageList,
    manager: glgp::manager::PackageManager,
}

impl GitLab {
    fn find_package(
        &mut self,
        name: &str,
        version: &str,
    ) -> Result<Option<glgp::types::PackageEntry>> {
        let mut page = 1;
        loop {
            let mut data = self.list.search(page, name).map_err(Error::Network)?;
            if data.len() == 0 {
                return Ok(None);
            }
            for i in 0..data.len() {
                if data[i].version == version {
                    return Ok(Some(data.remove(i)));
                }
            }
            page += 1;
        }
    }

    fn find_file(
        &mut self,
        package: &glgp::types::PackageEntry,
        file_name: &str,
    ) -> Result<Option<glgp::types::PackageFile>> {
        let mut page = 1;
        loop {
            let mut data = self
                .list
                .list_files(page, &package)
                .map_err(Error::Network)?;
            if data.len() == 0 {
                return Ok(None);
            }
            for i in 0..data.len() {
                if data[i].file_name == file_name {
                    return Ok(Some(data.remove(i)));
                }
            }
            page += 1;
        }
    }
}

fn download_file(target: &Path, src: &mut dyn Read) -> io::Result<()> {
    let mut buf: [u8; 8192] = [0; 8192];
    let mut f = File::create(target)?;

    loop {
        let bytes = src.read(&mut buf)?;
        if bytes == 0 {
            break;
        }
        f.write(&buf[0..bytes])?;
    }
    Ok(())
}

impl Source for GitLab {
    fn ensure_valid_package(&mut self, dep: &Dependency) -> Result<()> {
        let re = Regex::new(r"^\A\d+\.\d+\.\d+\z$").unwrap();
        let re1 = Regex::new(r"^([a-z]|[A-Z]|\d|\.|-|_)+$").unwrap();

        if !re.is_match(dep.version()) {
            error!("Invalid package version: {}", dep.version());
            return Err(Error::InvalidDep(dep.clone()));
        }
        if !re1.is_match(dep.name()) {
            error!("Invalid package name: {}", dep.name());
            return Err(Error::InvalidDep(dep.clone()));
        }
        Ok(())
    }

    fn publish(&mut self, dep: &Dependency, target: &str, src_file: &Path) -> Result<()> {
        if let Some(pkg) = self.find_package(dep.name(), dep.version())? {
            if let Some(_) = self.find_file(&pkg, target)? {
                error!({ target }, "Package {} already exists for target", dep);
                return Err(Error::AlreadyExists(dep.clone()));
            }
        }
        if self.manager.is_authenticated() {
            let f = File::open(&src_file).map_err(Error::Io)?;
            self.manager
                .upload(dep.name(), dep.version(), target, f)
                .map_err(Error::Network)?;
            return Ok(());
        }
        Err(Error::from(
            "The registry does not have a valid access token!",
        ))
    }

    fn find_latest(&mut self, name: &str) -> Result<Option<Dependency>> {
        let mut data = self.list.search(1, name).map_err(Error::Network)?;
        if data.len() == 0 {
            return Ok(None);
        }
        let p = data.remove(0);
        Ok(Some(Dependency::new(p.name, p.version)))
    }

    fn find(&mut self, name: &str, version: &str) -> Result<Option<Dependency>> {
        let package = self.find_package(&name, &version)?;
        if let Some(pkg) = package {
            return Ok(Some(Dependency::new(pkg.name, pkg.version)));
        }
        Ok(None)
    }

    fn download(&mut self, dep: &Dependency, target: &str, target_path: &Path) -> Result<()> {
        let pkg = glgp::types::PackageEntry {
            id: 0,
            version: dep.version().into(),
            name: dep.name().into(),
        };
        let file = glgp::types::PackageFile {
            id: 0,
            file_name: String::from(target),
            size: 0,
        };
        let mut response = self.manager.download(&pkg, &file).map_err(Error::Network)?;
        download_file(target_path, &mut response).map_err(Error::Io)?;
        Ok(())
    }
}

struct GitLabProvider;

impl Provider for GitLabProvider {
    fn open(&self, path: &str, params: &Parameters) -> Result<Box<dyn Source>> {
        let ppath = params
            .get("project-path")
            .ok_or(Error::MissingParameter("project-path"))?
            .as_str()
            .ok_or(Error::InvalidParameter("project-path"))?;
        ppath
            .find('/')
            .ok_or(Error::InvalidParameter("project-path"))?;
        let mut base_url = get_base_url(path);
        let pid = get_project_id(&base_url, ppath).map_err(Error::Network)?;
        base_url += &format!("/{}", pid);
        let allow_guest = params
            .get("allow-guest")
            .map(|v| v.as_boolean().ok_or(Error::InvalidParameter("allow-guest")))
            .unwrap_or(Ok(true))?;
        let token = match params.get("token") {
            Some(token) => Some(token.as_str().ok_or(Error::InvalidParameter("token"))?),
            None => None,
        };
        let ty = params.get("token-type")
            .map(|v| v.as_enum(&[("private", TokenType::Private), ("job", TokenType::Job)]))
            .unwrap_or(Some(TokenType::Private)).ok_or(Error::InvalidParameter("token-type"))?;
        if allow_guest {
            Ok(Box::new(GitLab {
                list: glgp::list::PackageList::new_guest(base_url.clone()),
                manager: token.map(|v| glgp::manager::PackageManager::new_authenticated(base_url.clone(), ty, v.into()))
                    .unwrap_or(glgp::manager::PackageManager::new_guest(base_url)),
            }))
        } else {
            let token = token.ok_or(Error::MissingParameter("token"))?;
            Ok(Box::new(GitLab {
                list: glgp::list::PackageList::new_authenticated(base_url.clone(), token.into()),
                manager: glgp::manager::PackageManager::new_authenticated(base_url, ty, token.into()),
            }))
        }
    }

    fn scheme(&self) -> &str {
        "gitlab"
    }
}

pub static GLGP: &dyn Provider = &GitLabProvider;
