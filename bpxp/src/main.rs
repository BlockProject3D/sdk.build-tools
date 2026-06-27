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

use std::fs::File;
use std::io::{Seek, Write};
use std::path::Path;
use bpx::package::Package;
use bpx::package::util::{pack_file, pack_file_vname, unpack};
use bpx::strings::get_name_from_path;
use clap::Parser;
use crate::args::Args;

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
        for f in args.file_names.iter().rev().skip(1).rev(){
            let name = f.to_str().ok_or(bpx::package::error::Error::Strings(bpx::strings::Error::Utf8))?;
            let obj = objects.find(name)?.expect(&format!("Unknown object with name {}", name));
            let out = File::create(destination.join(name))?;
            objects.load(obj, out)?;
        }
        Ok(())
    }
}

fn pack_file_rec<T: Seek + Write>(inner_path: String, path: &Path, package: &mut Package<T>) -> bpx::package::Result<()> {
    if path.is_dir() {
        for entry in path.read_dir()? {
            let entry = entry?;
            let inner_path = inner_path.clone() + "/" + entry.file_name().to_str().ok_or(bpx::package::error::Error::Strings(bpx::strings::Error::Utf8))?;
            pack_file_rec(inner_path, entry.path().as_path(), package)?;
        }
    } else {
        let fname = get_name_from_path(path)?;
        pack_file_vname(package, &(inner_path + "/" + fname), path)?;
    }
    Ok(())
}

fn compress(args: &Args) -> bpx::package::Result<()> {
    let mut package = Package::create(File::create(&args.file)?)?;
    for f in &args.file_names {
        pack_file_rec(".".into(), f, &mut package)?;
    }
    package.save()?;
    Ok(())
}

fn list(args: &Args) -> bpx::package::Result<()> {
    let package = Package::open(File::open(&args.file)?)?;
    let objects = package.objects()?;
    for obj in &objects {
        println!("{} ({} mbit(s))", objects.load_name(obj)?, (obj.size as f64) / 1024.0 / 1024.0);
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if args.extract {
        extract(&args).expect("Failed to extract package");
    } else if args.compress {
        compress(&args).expect("Failed to compress package");
    } else if args.list {
        list(&args).expect("Failed to list contents of package");
    } else {
        eprintln!("Please specify a mode (extract, compress or list)!");
        std::process::exit(1);
    }
}
