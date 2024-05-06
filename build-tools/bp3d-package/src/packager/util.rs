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

use std::error::Error;
use std::path::Path;
use std::process::Command;

pub trait CommandExt {
    fn ensure<E: Error + From<std::io::Error>>(&mut self, fail_value: E) -> Result<(), E>;

    fn output_string(&mut self) -> std::io::Result<String>;
}

impl CommandExt for Command {
    fn ensure<E: Error + From<std::io::Error>>(&mut self, fail_value: E) -> Result<(), E> {
        let res = self.status()?;
        if !res.success() {
            Err(fail_value)
        } else {
            Ok(())
        }
    }

    fn output_string(&mut self) -> std::io::Result<String> {
        Ok(String::from_utf8_lossy(&self.output()?.stdout).into())
    }
}

pub fn ensure_clean_directories<'a>(directories: impl IntoIterator<Item = &'a Path>) -> std::io::Result<()> {
    for dir in directories {
        if dir.exists() {
            std::fs::remove_dir_all(dir)?;
        }
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

macro_rules! typed_ident {
    ($t: ty, $name: ident) => { $name };
}

macro_rules! packager_error {
    (
        $name: ident {
            $($ty: ident $(($data: ty))? => $desc: literal),*
        }
    ) => {
        #[derive(Debug)]
        pub enum $name {
            Io(std::io::Error),
            $($ty $(($data))?),*
        }

        impl From<std::io::Error> for $name {
            fn from(value: std::io::Error) -> Self {
                Self::Io(value)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $name::Io(e) => write!(f, "io error: {}", e),
                    $($name::$ty $((crate::packager::util::typed_ident!($data, e)))? => write!(f, $desc $(, crate::packager::util::typed_ident!($data, e))?) ),*
                }
            }
        }

        impl std::error::Error for $name {}
    };
}

macro_rules! packager_registry {
    ($($module: ident::$name: ident),*) => {
        $(mod $module;)*

        #[derive(clap::ValueEnum, Debug, Copy, Clone)]
        pub enum PackagerType {
            $($name),*
        }

        impl PackagerType {
            pub fn call<P: crate::packager::interface::Package>(&self, context: &crate::packager::interface::Context<P>) {
                match self {
                    $(PackagerType::$name => crate::core::run_packager::<P, $module::$name>(context))*
                }
            }
        }
    };
}

pub(crate) use packager_registry;
pub(crate) use typed_ident;
pub(crate) use packager_error;
