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
use std::ffi::{OsStr, OsString};
use std::path::Path;
use bp3d_util::simple_error;
use serde::Deserialize;
use toml::value::Datetime;

#[derive(Deserialize)]
pub enum ParamValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Datetime(Datetime)
}

pub type Parameters = HashMap<String, ParamValue>;

#[derive(Deserialize)]
pub struct Source {
    pub url: OsString,
    pub params: Parameters
}

impl Source {
    pub fn scheme(&self) -> Option<&str> {
        let id = self.url.as_os_str().as_encoded_bytes().iter().position(|b| *b == b':')?;
        std::str::from_utf8(&self.url.as_encoded_bytes()[..id]).ok()
    }

    pub fn path(&self) -> &OsStr {
        let id = self.url.as_os_str().as_encoded_bytes().iter().position(|b| *b == b':');
        match id {
            Some(id) => {
                let bytes = &self.url.as_os_str().as_encoded_bytes()[id + 1..];
                if bytes.len() > 2 && bytes[0] == b'/' && bytes[1] == b'/' {
                    // Safety: This is only constructured from as_ecnoded_bytes and after a valid ascii comparison
                    unsafe { OsStr::from_encoded_bytes_unchecked(&bytes[2..]) }
                } else {
                    // Safety: This is only constructured from as_ecnoded_bytes
                    unsafe { OsStr::from_encoded_bytes_unchecked(&bytes) }
                }
            },
            None => self.url.as_os_str()
        }
    }
}

#[derive(Deserialize)]
pub struct Dependency {
    pub source: String,
    pub version: String
}

#[derive(Deserialize)]
pub struct Config {
    /// Represents the default package source for publishing new packages.
    #[serde(rename="default")]
    pub default_source: Option<String>,

    /// The list of dependencies to be installed.
    pub dependencies: HashMap<String, Dependency>,

    /// A declaration of all available package sources.
    pub sources: HashMap<String, Source>
}

simple_error! {
    pub Error {
        Toml(toml::de::Error) => "toml error: {}",
        Io(std::io::Error) => "io error: {}",
        Missing => "missing configuration for fpkg"
    }
}

#[derive(Deserialize)]
pub struct ManifestExt {
    pub fpkg: Config
}

pub fn parse_config(project_root: &Path) -> Result<Config, Error> {
    let path = project_root.join("bp3d.toml");
    if path.exists() && path.is_file() {
        let data = std::fs::read(path).map_err(Error::Io)?;
        let config: ManifestExt = toml::from_slice(&data).map_err(Error::Toml)?;
        return Ok(config.fpkg);
    }
    Err(Error::Missing)
}
