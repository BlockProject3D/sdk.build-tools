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

use std::io::Write;
use std::marker::PhantomData;
use bp3d_proto::message::{FromBytes, Message, WriteTo};
use bp3d_proto::util::ToUsize;
use bp3d_proto::message::Result;

pub struct VarBytes<T>(PhantomData<T>);

impl<'a, T: FromBytes<'a, Output: ToUsize>> FromBytes<'a> for VarBytes<T> {
    type Output = &'a [u8];

    fn from_bytes(slice: &'a [u8]) -> Result<Message<Self::Output>> {
        let msg = T::from_bytes(slice)?;
        let size = msg.size();
        let len = msg.into_inner().to_usize();
        Ok(Message::new(size + len, &slice[size..len + size]))
    }
}

impl<'a, T: WriteTo<Input<'a>: ToUsize>> WriteTo for VarBytes<T> {
    type Input<'b> = &'b [u8];

    fn write_to<W: Write>(input: &Self::Input<'_>, mut out: W) -> Result<()> {
        let len = input.len();
        T::write_to(&T::Input::from_usize(len), &mut out)?;
        out.write_all(input)?;
        Ok(())
    }
}
