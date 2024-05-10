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
use std::path::{Path, PathBuf};
use crate::output::Output;

pub trait Context {
    fn get_target_path(&self, target: &str) -> Cow<'_, Path>;
    fn get_outputs(&self) -> impl Iterator<Item = Output>;
}

pub struct FinderResult {
    pub path: Option<PathBuf>,
    pub debug_info: Option<PathBuf>,
    pub exports: Option<PathBuf>
}

pub enum LibType {
    Dynamic,
    Static
}

pub struct Finder<'a, C> {
    context: &'a C,
    target: &'a str
}

impl<'a, C: Context> Finder<'a, C> {
    pub fn new(context: &'a C, target: &'a str) -> Self {
        Self {
            context,
            target
        }
    }

    pub fn get_path(&self, file_name: &str) -> Option<PathBuf> {
        let path = self.context.get_target_path(self.target).join(file_name);
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    pub fn find_first<F: Fn(&Output) -> bool>(&self, lib_type: LibType, predicate: F) -> FinderResult {
        let value = self.context.get_outputs().find(predicate);
        match value {
            None => FinderResult {
                path: None,
                debug_info: None,
                exports: None
            },
            Some(v) => self.resolve_output(lib_type, &v)
        }
    }

    pub fn resolve_output_all(&self, output: &Output) -> Vec<FinderResult> {
        let mut results = Vec::new();
        if output.is_lib() {
            let rstatic = self.resolve_output(LibType::Static, output);
            let rdynamic = self.resolve_output(LibType::Dynamic, output);
            if rstatic.path.is_some() {
                results.push(rstatic);
            }
            if rdynamic.path.is_some() {
                results.push(rdynamic);
            }
        } else {
            let rdynamic = self.resolve_output(LibType::Dynamic, output);
            if rdynamic.path.is_some() {
                results.push(rdynamic);
            }
        }
        results
    }

    pub fn resolve_output(&self, lib_type: LibType, output: &Output) -> FinderResult {
        match output {
            Output::Bin(name) => {
                #[cfg(unix)]
                return FinderResult {
                    path: self.get_path(name),
                    debug_info: self.get_path(&format!("{}.d", name)),
                    exports: None
                };
                #[cfg(windows)]
                return FinderResult {
                    path: self.get_path(&format!("{}.exe", name)),
                    debug_info: self.get_path(&format!("{}.pdb", name)),
                    exports: None
                };
            },
            Output::Lib(name) => {
                match lib_type {
                    LibType::Dynamic => {
                        #[cfg(unix)]
                        return FinderResult {
                            path: self.get_path(&format!("lib{}.dylib", name))
                                .or_else(|| self.get_path(&format!("lib{}.so", name)))
                                .or_else(|| self.get_path(&format!("{}.dylib", name)))
                                .or_else(|| self.get_path(&format!("{}.so", name))),
                            debug_info: self.get_path(&format!("lib{}.d", name))
                                .or_else(|| self.get_path(&format!("{}.d", name))),
                            exports: None
                        };
                        #[cfg(windows)]
                        return FinderResult {
                            path: self.get_path(&format!("{}.dll", name))
                                .or_else(|| self.get_path(&format!("lib{}.dll", name))),
                            debug_info: self.get_path(&format!("{}.pdb", name))
                                .or_else(|| self.get_path(&format!("lib{}.pdb", name))),
                            exports: self.get_path(&format!("{}.dll.lib", name))
                                .or_else(|| self.get_path(&format!("lib{}.dll.lib", name)))
                                .or_else(|| self.get_path(&format!("{}.lib", name)))
                                .or_else(|| self.get_path(&format!("lib{}.lib", name)))
                        };
                    },
                    LibType::Static => {
                        #[cfg(unix)]
                        return FinderResult {
                            path: self.get_path(&format!("lib{}.a", name))
                                .or_else(|| self.get_path(&format!("{}.a", name))),
                            debug_info: self.get_path(&format!("lib{}.d", name))
                                .or_else(|| self.get_path(&format!("{}.d", name))),
                            exports: None
                        };
                        #[cfg(windows)]
                        return FinderResult {
                            path: self.get_path(&format!("{}.lib", name))
                                .or_else(|| self.get_path(&format!("lib{}.lib", name))),
                            debug_info: self.get_path(&format!("{}.pdb", name))
                                .or_else(|| self.get_path(&format!("lib{}.pdb", name))),
                            exports: None
                        };
                    }
                }
            },
            Output::Config(name) => FinderResult {
                path: self.get_path(name),
                debug_info: None,
                exports: None
            },
            Output::Other(name) => FinderResult {
                path: self.get_path(name),
                debug_info: None,
                exports: None
            }
        }
    }
}
