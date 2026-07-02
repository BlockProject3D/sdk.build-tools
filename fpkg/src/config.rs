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
use std::path::Path;
use bp3d_util::simple_error;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ParamValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool)
}

impl ParamValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            ParamValue::String(s) => Some(s.as_str()),
            _ => None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            ParamValue::Integer(v) => Some(*v as _),
            ParamValue::Float(v) => Some(*v),
            _ => None
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ParamValue::Integer(v) => Some(*v),
            ParamValue::Float(v) => Some(*v as _),
            _ => None
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            ParamValue::Boolean(v) => Some(*v),
            _ => None
        }
    }
}

pub type Parameters = HashMap<String, ParamValue>;

#[derive(Deserialize)]
pub struct Source {
    pub url: String,
    pub params: Parameters
}

impl Source {
    pub fn scheme(&self) -> Option<&str> {
        let id = self.url.as_bytes().iter().position(|b| *b == b':')?;
        Some(&self.url[..id])
    }

    pub fn path(&self) -> &str {
        let id = self.url.as_bytes().iter().position(|b| *b == b':');
        match id {
            Some(id) => {
                let bytes = &self.url[id + 1..];
                if bytes.len() > 2 && bytes.as_bytes()[0] == b'/' && bytes.as_bytes()[1] == b'/' {
                    &bytes[2..]
                } else {
                    bytes
                }
            },
            None => &self.url
        }
    }
}

#[derive(Deserialize)]
pub struct Dependency {
    pub source: String,
    pub version: String
}

#[derive(Deserialize, Default)]
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
        Io(std::io::Error) => "io error: {}"
    }
}

#[derive(Deserialize)]
pub struct ManifestExt {
    pub fpkg: Config
}

pub fn parse_config(project_root: &Path) -> Result<Option<Config>, Error> {
    let path = project_root.join("bp3d.toml");
    if path.exists() && path.is_file() {
        let data = std::fs::read(path).map_err(Error::Io)?;
        let config: ManifestExt = toml::from_slice(&data).map_err(Error::Toml)?;
        return Ok(Some(config.fpkg));
    }
    Ok(None)
}

pub fn parse_standalone_config(path: &Path) -> Result<Config, Error> {
    let data = std::fs::read(path).map_err(Error::Io)?;
    let config: Config = toml::from_slice(&data).map_err(Error::Toml)?;
    Ok(config)
}
