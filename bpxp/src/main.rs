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

use crate::args::Args;
use bp3d_util::result::ResultExt;
use bpx::package::util::{pack_file_vname, unpack};
use bpx::package::{Architecture, CreateOptions, Package, Platform};
use bpx::sd::debug::Debugger;
use bpx::sd::formatting::{Format, IndentType};
use bpx::sd::{Object, Value};
use clap::Parser;
use std::fs::File;

mod args;

fn extract(args: &Args) -> bpx::package::Result<()> {
    if args.file_names.is_empty() {
        eprintln!("Please specify at least an output directory");
        std::process::exit(1);
    }
    let package = Package::open(File::open(&args.file)?)?;
    if args.file_names.len() == 1 {
        unpack(&package, &args.file_names.last().unwrap())
    } else {
        let destination = args.file_names.last().unwrap();
        let objects = package.objects()?;
        for f in args.file_names.iter().rev().skip(1).rev() {
            let name = f.to_str().ok_or(bpx::package::error::Error::Strings(
                bpx::strings::Error::Utf8,
            ))?;
            let obj = objects
                .find(name)?
                .expect(&format!("Unknown object with name {}", name));
            let out = File::create(destination.join(name))?;
            objects.load(obj, out)?;
        }
        Ok(())
    }
}

fn get_platform_from_target(target: &str) -> Platform {
    if target.contains("windows") {
        Platform::Windows
    } else if target.contains("apple") {
        Platform::Mac
    } else if target.contains("linux") {
        Platform::Linux
    } else {
        Platform::Any
    }
}

fn get_architecture_from_target(target: &str) -> Architecture {
    if target.contains("x86_64") {
        Architecture::X86_64
    } else if target.contains("x86") {
        Architecture::X86
    } else if target.contains("aarch64") || target.contains("arm64") {
        Architecture::Aarch64
    } else if target.contains("arm") {
        Architecture::Armv7hl
    } else {
        Architecture::Any
    }
}

fn compress(args: &Args) -> bpx::package::Result<()> {
    let mut opts = CreateOptions::new(File::create(&args.file)?);
    if let Some(target) = &args.target {
        opts = opts
            .architecture(get_architecture_from_target(target))
            .platform(get_platform_from_target(target));
    }
    if !args.metadata.is_empty() {
        let mut obj = Debugger::attach(Object::with_capacity(args.metadata.len() as _)).unwrap();
        for kv in &args.metadata {
            let mut kv = kv.split("=");
            let key = kv.next();
            let value = kv.next();
            match (key, value) {
                (Some(key), Some(value)) => {
                    if let Ok(v) = value.parse::<i32>() {
                        obj.set(key, Value::Int32(v));
                    } else if let Ok(v) = value.parse::<f32>() {
                        obj.set(key, Value::Float(v));
                    } else if value == "true" {
                        obj.set(key, Value::Bool(true));
                    } else if value == "false" {
                        obj.set(key, Value::Bool(false));
                    } else {
                        obj.set(key, Value::String(value.to_string()));
                    }
                }
                _ => continue,
            }
        }
        opts = opts.metadata(Value::Object(obj.detach()));
    }
    if let Some(type_code) = &args.type_code {
        if type_code.len() == 2 {
            opts = opts.type_code([type_code.as_bytes()[0], type_code.as_bytes()[1]]);
        } else {
            eprintln!(
                "Cannot set type code {}: type code must be a 2 bytes ASCII string",
                type_code
            );
        }
    } else {
        opts = opts.type_code([0x50, 0x4B]);
    }
    let mut package = Package::create(opts)?;
    for f in &args.file_names {
        let vname = f.to_str().ok_or(bpx::package::error::Error::Strings(
            bpx::strings::Error::Utf8,
        ))?;
        pack_file_vname(&mut package, vname, f)?;
    }
    package.save()?;
    Ok(())
}

fn info(args: &Args) -> bpx::package::Result<()> {
    let package = Package::open(File::open(&args.file)?)?;
    println!("Platform: {:?}", package.settings().platform);
    println!("Architecture: {:?}", package.settings().architecture);
    println!();
    let metadata = package.load_metadata()?;
    if let Value::Object(obj) = metadata {
        println!("==> Metadata <==");
        let fmt = obj.format(IndentType::Spaces, 4);
        println!("{}", fmt);
        println!();
    }
    println!("==> Objects <==");
    let objects = package.objects()?;
    for obj in &objects {
        println!(
            "{} ({} mbit(s))",
            objects.load_name(obj)?,
            (obj.size as f64) / 1024.0 / 1024.0
        );
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if args.extract {
        extract(&args).expect_exit("Failed to extract package", 1);
    } else if args.compress {
        compress(&args).expect_exit("Failed to compress package", 1);
    } else if args.info {
        info(&args).expect_exit("Failed to dump contents of package", 1);
    } else {
        eprintln!("Please specify a mode (extract, compress or info)!");
        std::process::exit(1);
    }
}
