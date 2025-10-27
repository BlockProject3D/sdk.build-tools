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

use std::path::{Path, PathBuf};
use crate::system::finder::Finder;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum LibType {
    Dynamic,
    Static
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Type {
    Bin,
    Lib(LibType),
    Header,
    Config,
    Other
}

#[derive(Clone)]
pub struct Artifact {
    path: PathBuf,
    debug_info: Option<PathBuf>,
    exports: Option<PathBuf>,
    name: String,
    ty: Type
}

impl Artifact {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn debug_info(&self) -> Option<&Path> {
        self.debug_info.as_deref()
    }

    pub fn exports(&self) -> Option<&Path> {
        self.exports.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> Type {
        self.ty
    }

    pub fn find_bin(path: &Path, name: &str) -> Option<Self> {
        let res = Finder::new(path).find_bin(name);
        Some(Self {
            path: res.path?,
            debug_info: res.debug_info,
            exports: res.exports,
            name: name.into(),
            ty: Type::Bin
        })
    }

    pub fn find_lib(path: &Path, name: &str, ty: LibType) -> Option<Self> {
        let res = Finder::new(path).find_lib(name, ty);
        Some(Self {
            path: res.path?,
            debug_info: res.debug_info,
            exports: res.exports,
            name: name.into(),
            ty: Type::Lib(ty)
        })
    }

    pub fn header(path: &Path, name: &str) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            debug_info: None,
            exports: None,
            ty: Type::Header
        }
    }

    pub fn config(path: &Path, name: &str) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            debug_info: None,
            exports: None,
            ty: Type::Config
        }
    }

    pub fn other(path: &Path, name: &str) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            debug_info: None,
            exports: None,
            ty: Type::Other
        }
    }
}

#[derive(Clone)]
pub struct List {
    content: Vec<Artifact>,
}

impl List {
    pub fn new() -> Self {
        Self { content: Vec::new() }
    }

    pub fn add_if_some(&mut self, artifact: Option<Artifact>) {
        if let Some(artifact) = artifact {
            self.add(artifact);
        }
    }

    pub fn add(&mut self, artifact: Artifact) {
        let val = self.content.iter().any(|v| v.name == artifact.name);
        if val {
            panic!("Duplicate artifact {} found", artifact.name);
        }
        self.content.push(artifact);
    }

    pub fn find(&self, ty: Type) -> impl Iterator<Item = &Artifact> {
        self.content.iter().filter(move |v| v.ty == ty)
    }

    pub fn find_first(&self, ty: Type) -> Option<&Artifact> {
        self.content.iter().filter(move |v| v.ty == ty).next()
    }

    pub fn into_inner(self) -> Vec<Artifact> {
        self.content
    }
}
