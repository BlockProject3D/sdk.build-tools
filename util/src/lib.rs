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

pub trait ResultExt<T> {
    fn expect_exit(self, msg: &str, code: i32) -> T;
}

impl<T, E: Error> ResultExt<T> for Result<T, E> {
    fn expect_exit(self, msg: &str, code: i32) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {}", msg, e);
                std::process::exit(code);
            }
        }
    }
}

#[macro_export]
macro_rules! typed_ident {
    ($t: ty, $name: ident) => { $name };
}

#[macro_export]
macro_rules! simple_error {
    (
        $name: ident {
            $($((impl From))? $ty: ident $(($data: ty))? => $desc: literal),*
        }
    ) => {
        #[derive(Debug)]
        pub enum $name {
            $($ty $(($data))?),*
        }

        $(
            $(
                impl From<$data> for $name {
                    fn from(value: $data) -> Self {
                        Self::$ty(value)
                    }
                }
            )?
        )*

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$ty $(($crate::typed_ident!($data, e)))? => write!(f, $desc $(, $crate::typed_ident!($data, e))?) ),*
                }
            }
        }

        impl std::error::Error for $name {}
    };
}