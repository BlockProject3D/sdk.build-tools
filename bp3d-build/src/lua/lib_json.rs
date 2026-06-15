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

use std::fmt::{Display, Formatter};
use bp3d_lua::decl_lib_func;
use bp3d_lua::libs::Lib;
use bp3d_lua::util::Namespace;
use bp3d_lua::vm::function::types::RFunction;
use bp3d_lua::vm::table::Table;
use bp3d_lua::vm::value::any::Any;
use bp3d_lua::vm::Vm;
use bp3d_util::simple_error;
use json_deserializer::{Number, Object, Value};

/*#[derive(Debug)]
struct Error(json_deserializer::Error);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}*/

simple_error! {
    pub Error {
        Json(json_deserializer::Error) => "json error: {}",
        Lua(bp3d_lua::vm::error::Error) => "lua error: {}",
        ParseFloat(std::num::ParseFloatError) => "float parse error: {}",
        ParseInt(std::num::ParseIntError) => "integer parse error: {}",
        Utf8(std::str::Utf8Error) => "utf8 error: {}"
    }
}

pub trait Pow { // A fixed variant of the pow trait (the one of Rust is broken)!!
    fn pow(self, other: Self) -> Self;
}

impl Pow for f64 {
    fn pow(self, other: Self) -> Self {
        self.powf(other)
    }
}

impl Pow for i64 {
    fn pow(self, other: Self) -> Self {
        self.pow(other as _)
    }
}

fn parse_num<'a>(num: Number<'a>) -> Result<Any<'a>, Error> {
    match num {
        Number::Float(value, exp) => {
            let value: f64 = std::str::from_utf8(value).map_err(Error::Utf8)?.parse().map_err(Error::ParseFloat)?;
            let mut exp2 = 0.0;
            if !exp.is_empty() {
                exp2 = std::str::from_utf8(exp).map_err(Error::Utf8)?.parse().map_err(Error::ParseFloat)?;
            }
            if exp2 != 0.0 {
                Ok(Any::Number(value * 10.0.pow(exp2)))
            } else {
                Ok(Any::Number(value))
            }
        }
        Number::Integer(value, exp) => {
            let value: i64 = std::str::from_utf8(value).map_err(Error::Utf8)?.parse().map_err(Error::ParseInt)?;
            let mut exp2: i64 = 0;
            if !exp.is_empty() {
                exp2 = std::str::from_utf8(exp).map_err(Error::Utf8)?.parse().map_err(Error::ParseInt)?;
            }
            if exp2 != 0 {
                Ok(Any::Int64(value * 10.pow(exp2)))
            } else {
                Ok(Any::Int64(value))
            }
        }
    }
}

fn parse_value<'a>(vm: &'a Vm, value: Value<'a>) -> Result<Any<'a>, Error> {
    match value {
        Value::Null => Ok(Any::Nil),
        Value::String(s) => Ok(Any::String(s)),
        Value::Bool(v) => Ok(Any::Boolean(v)),
        Value::Object(obj) => parse_object(vm, obj).map(Any::Table),
        Value::Number(num) => parse_num(num),
        Value::Array(arr) => parse_array(vm, arr).map(Any::Table)
    }
}

fn parse_object<'a>(vm: &'a Vm, obj: Object) -> Result<Table<'a>, Error> {
    let mut tbl = Table::with_capacity(vm, 0, obj.len());
    for (k, v) in obj.into_iter() {
        let value = parse_value(vm, v)?;
        tbl.set(&*k, value).map_err(Error::Lua)?;
    }
    Ok(tbl)
}

fn parse_array<'a>(vm: &'a Vm, arr: Vec<Value<'_>>) -> Result<Table<'a>, Error> {
    let mut tbl = Table::with_capacity(vm, arr.len(), 0);
    for v in arr.into_iter() {
        let value = parse_value(vm, v)?;
        tbl.push(value).map_err(Error::Lua)?;
    }
    Ok(tbl)
}

decl_lib_func! {
    fn json_decode<'a>(vm: &Vm, data: &'a [u8]) -> Result<Any<'a>, Error> {
        let data = json_deserializer::parse(data).map_err(Error::Json)?;
        parse_value(vm, data)
    }
}

pub struct JsonLib;

impl Lib for JsonLib {
    const NAMESPACE: &'static str = "bp3d.build.json";

    fn load(&self, namespace: &mut Namespace) -> bp3d_lua::vm::Result<()> {
        namespace.add([
            ("decode", RFunction::wrap(json_decode))
        ])
    }
}
