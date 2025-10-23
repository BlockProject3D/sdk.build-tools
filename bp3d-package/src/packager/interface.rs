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

use std::error::Error;
use std::path::{Path, PathBuf};
use bp3d_build::core::BuildTool;
use bp3d_build::system::artifact::List;
use bp3d_build::system::Features;

pub struct Context<'a> {
    pub path: &'a Path,
    pub configuration: &'a str,
    pub targets: &'a [&'a str],
    pub tool: &'a dyn BuildTool
}

impl<'a> Context<'a> {
    pub fn get_target_path(&self, target: &str) -> PathBuf {
        self.path.join("target").join(target).join(self.configuration)
    }
}

pub trait Packager {
    const NAME: &'static str;

    type Error: Error + From<bp3d_build::core::Error>;

    fn do_build_target(&self, target: &str, context: &Context) -> Result<List, Self::Error> {
        let ctx = bp3d_build::system::Context {
            path: context.path,
            target,
            configuration: context.configuration,
            features: Features::All
        };
        context.tool.configure(&ctx)?;
        let data = context.tool.pre_package(&ctx)?;
        Ok(data)
    }

    fn do_build(&self, _context: &Context) -> Result<(), Self::Error> {
        Ok(())
    }

    fn do_package_target(&self, _list: &List, _target: &str, _context: &Context) -> Result<(), Self::Error> {
        Ok(())
    }

    fn do_package(&self, _context: &Context) -> Result<(), Self::Error> {
        Ok(())
    }
}
