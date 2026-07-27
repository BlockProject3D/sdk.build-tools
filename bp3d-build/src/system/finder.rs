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

use crate::system::artifact::LibType;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FinderResult {
    pub path: Option<PathBuf>,
    pub debug_info: Option<PathBuf>,
    pub exports: Option<PathBuf>,
}

pub struct Finder<'a> {
    root: &'a Path,
}

impl<'a> Finder<'a> {
    pub fn new(root: &'a Path) -> Self {
        Self { root }
    }

    pub fn get_path(&self, file_name: &str) -> Option<PathBuf> {
        let path = self.root.join(file_name);
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    pub fn find_bin(&self, name: &str) -> FinderResult {
        #[cfg(unix)]
        return FinderResult {
            path: self.get_path(name),
            debug_info: self.get_path(&format!("{}.d", name)),
            exports: None,
        };
        #[cfg(windows)]
        return FinderResult {
            path: self.get_path(&format!("{}.exe", name)),
            debug_info: self.get_path(&format!("{}.pdb", name)),
            exports: None,
        };
    }

    pub fn find_lib(&self, name: &str, lib_type: LibType) -> FinderResult {
        match lib_type {
            LibType::Dynamic => {
                #[cfg(unix)]
                return FinderResult {
                    path: self
                        .get_path(&format!("lib{}.dylib", name))
                        .or_else(|| self.get_path(&format!("lib{}.so", name)))
                        .or_else(|| self.get_path(&format!("{}.dylib", name)))
                        .or_else(|| self.get_path(&format!("{}.so", name))),
                    debug_info: self
                        .get_path(&format!("lib{}.d", name))
                        .or_else(|| self.get_path(&format!("{}.d", name))),
                    exports: None,
                };
                #[cfg(windows)]
                return FinderResult {
                    path: self
                        .get_path(&format!("{}.dll", name))
                        .or_else(|| self.get_path(&format!("lib{}.dll", name))),
                    debug_info: self
                        .get_path(&format!("{}.pdb", name))
                        .or_else(|| self.get_path(&format!("lib{}.pdb", name))),
                    exports: self
                        .get_path(&format!("{}.dll.lib", name))
                        .or_else(|| self.get_path(&format!("lib{}.dll.lib", name)))
                        .or_else(|| self.get_path(&format!("{}.lib", name)))
                        .or_else(|| self.get_path(&format!("lib{}.lib", name))),
                };
            }
            LibType::Static => {
                #[cfg(unix)]
                return FinderResult {
                    path: self
                        .get_path(&format!("lib{}.a", name))
                        .or_else(|| self.get_path(&format!("{}.a", name))),
                    debug_info: self
                        .get_path(&format!("lib{}.d", name))
                        .or_else(|| self.get_path(&format!("{}.d", name))),
                    exports: None,
                };
                #[cfg(windows)]
                return FinderResult {
                    path: self
                        .get_path(&format!("{}.lib", name))
                        .or_else(|| self.get_path(&format!("lib{}.lib", name))),
                    debug_info: self
                        .get_path(&format!("{}.pdb", name))
                        .or_else(|| self.get_path(&format!("lib{}.pdb", name))),
                    exports: None,
                };
            }
        }
    }
}
