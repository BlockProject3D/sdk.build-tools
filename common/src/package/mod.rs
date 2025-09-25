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

mod interface;
mod cargo;
mod make;

pub use interface::*;

use std::path::Path;

impl<'a> Context<'a> {
    pub fn load(root: &'a Path, config: &'a str) -> Result<Context<'a>, Error> {
        let mut res = match cargo::Cargo::load(root, config) {
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        if let None = res {
            res = match make::Make::load(root, config) {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
        }
        let res = match res {
            Some(v) => v,
            None => return Err(Error::UnknownPackage)
        };
        Ok(res)
    }

    pub fn pre_build(&self, target: &str) -> Result<(), Error> {
        if !self.package.is_valid_target(target) {
            return Err(Error::InvalidTarget(target.into()))
        }
        self.package.pre_build(self, target)
    }
}
