// Copyright (c) 2024, BlockProject 3D
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
use std::error::Error;
use std::path::{Path, PathBuf};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "path")]
pub enum Output<'a> {
    Bin(Cow<'a, str>),
    Lib(Cow<'a, str>),
    Config(Cow<'a, str>),
    Other(Cow<'a, str>)
}

impl<'a> Output<'a> {
    pub fn name(&self) -> &str {
        match self {
            Output::Bin(v) => v,
            Output::Lib(v) => v,
            Output::Config(v) => v,
            Output::Other(v) => v
        }
    }
}

pub struct OutputList<'a> {
    outputs: Vec<Output<'a>>,
    paths: Vec<PathBuf>
}

impl<'a> OutputList<'a> {
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
            paths: Vec::new()
        }
    }

    pub fn add_target_path(&mut self, path: PathBuf) {
        self.paths.push(path);
    }

    pub fn add_bin(&mut self, name: impl Into<Cow<'a, str>>) {
        self.outputs.push(Output::Bin(name.into()));
    }
    pub fn add_lib(&mut self, name: impl Into<Cow<'a, str>>) {
        self.outputs.push(Output::Lib(name.into()));
    }
    pub fn add_config(&mut self, name: impl Into<Cow<'a, str>>) {
        self.outputs.push(Output::Config(name.into()));
    }
    pub fn add_other(&mut self, name: impl Into<Cow<'a, str>>) {
        self.outputs.push(Output::Other(name.into()));
    }
}

impl<'a> AsRef<Vec<Output<'a>>> for OutputList<'a> {
    fn as_ref(&self) -> &Vec<Output<'a>> {
        &self.outputs
    }
}

pub struct Module<'a> {
    pub name: &'a str,
    pub path: &'a Path
}

pub struct Context<'a> {
    pub root: &'a Path,
    pub target: Option<&'a str>,
    pub release: bool,
    pub all_features: bool,
    pub features: &'a[&'a str]
}

impl<'a> Context<'a> {
    pub fn target(&self) -> &str {
        self.target.unwrap_or(current_platform::CURRENT_PLATFORM)
    }

    pub fn get_dynlib_extension(&self) -> &str {
        if self.target().contains("apple") {
            ".dylib"
        } else if self.target().contains("windows") {
            ".dll"
        } else {
            ".so"
        }
    }

    pub fn get_staticlib_extension(&self) -> &str {
        if self.target().contains("windows") {
            ".lib"
        } else {
            ".a"
        }
    }

    pub fn get_exe_extension(&self) -> &str {
        if self.target().contains("windows") {
            ".exe"
        } else {
            ""
        }
    }
}

pub trait Builder: Sized {
    const NAME: &'static str;
    type Error: Error;

    fn do_configure(context: &Context, module: &Module) -> Result<Self, Self::Error>;
    fn do_compile(&self, context: &Context, module: &Module) -> Result<(), Self::Error>;
    fn list_outputs(&self, context: &Context, module: &Module) -> Result<OutputList, Self::Error>;
}
