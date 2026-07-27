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
use bp3d_util::simple_error;
use std::fmt::Display;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Dependency {
    version: String,
    name: String,
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.name, self.version)
    }
}

impl Dependency {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }

    pub fn get_package_filename(&self) -> String {
        format!("{}-{}.bpx", self.name, self.version)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

simple_error! {
    pub Error {
        MissingDep(Dependency) => "could not find dependency {}",
        Io(std::io::Error) => "io error: {}",
        InvalidPath => "invalid URL path",
        InvalidParameter(&'static str) => "invalid value for parameter: {}",
        MissingParameter(&'static str) => "missing value for parameter: {}",
        Network(reqwest::Error) => "network error: {}",
        InvalidDep(Dependency) => "unsupported dependency specification ({}) for package source",
        AlreadyExists(Dependency) => "dependency already exists: {}",
        Other(String) => "{}"
    }
}

impl<T: Into<String>> From<T> for Error {
    fn from(value: T) -> Self {
        Error::Other(value.into())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Source {
    /// Ensure the given dependency name/version format is compatible with this package source for
    /// publishing.
    ///
    /// # Arguments
    ///
    /// * `dep`: the dependency information which is going to be published.
    ///
    /// returns: Result<(), Error>
    fn ensure_valid_package(&mut self, dep: &Dependency) -> Result<()>;

    /// Publish a package.
    ///
    /// # Arguments
    ///
    /// * `dep`: the dependency which is being packaged.
    /// * `target`: the target triple.
    /// * `src_file`: the source BPX file to be uploaded.
    ///
    /// returns: Result<(), Error>
    fn publish(&mut self, dep: &Dependency, target: &str, src_file: &Path) -> Result<()>;

    /// Find the latest version of the given dependency name.
    ///
    /// # Arguments
    ///
    /// * `name`: the name of the dependency.
    ///
    /// returns: Result<Option<Dependency>, Error>
    fn find_latest(&mut self, name: &str) -> Result<Option<Dependency>>;

    fn find(&mut self, name: &str, version: &str) -> Result<Option<Dependency>>;

    /// Downloads the given dependency from this package source.
    ///
    /// # Arguments
    ///
    /// * `dep`: the dependency to be downloaded.
    /// * `target`: the destination system target tripple.
    /// * `target_path`: the path to the output BPX file.
    ///
    /// returns: Result<(), Error>
    fn download(&mut self, dep: &Dependency, target: &str, target_path: &Path) -> Result<()>;
}

pub trait Provider: Send + Sync {
    fn open(&self, path: &str, params: &Parameters) -> Result<Box<dyn Source>>;

    fn scheme(&self) -> &str;
}
