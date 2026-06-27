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

use std::io::Cursor;
use std::path::Path;
use bpx::package::Package;
use bpx::package::util::unpack;

#[derive(Default)]
pub struct Installer {
    name: &'static str,
    version: &'static str,
    package: &'static [u8]
}

impl Installer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn package(mut self, package: &'static [u8]) -> Self {
        self.package = package;
        self
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    pub fn version(mut self, version: &'static str) -> Self {
        self.version = version;
        self
    }

    pub fn run(self) {
        let mut args = std::env::args();
        let installer = args.next().unwrap();
        let subcmd = args.next().expect(&format!("Usage: {} <install/list/info> [optional install prefix]", installer));
        if subcmd == "install" {
            let install_name = String::from(self.name) + "-" + &self.version;
            let pack = Package::open(Cursor::new(self.package)).expect("Unable to open embedded installer package");
            let install_path = Path::new(&args.next().unwrap_or("/opt".into())).join(install_name);
            unpack(&pack, &install_path).expect("Failed to extract application objects");
        } else if subcmd == "list" {
            let pack = Package::open(Cursor::new(self.package)).expect("Unable to open embedded installer package");
            let objects = pack.objects().expect("Unable to read embedded installer package");
            println!("Installer {} - Objects:", installer);
            for obj in &objects {
                let name = objects.load_name(obj).expect("Unable to read name of object");
                println!("    > {}: {} kbit(s)", name, obj.size / 1024)
            }
        } else if subcmd == "info" {
            println!("Installer {}:", installer);
            println!("    > App name: {}", self.name);
            println!("    > App version: {}", self.version);
            println!("    > Size of package: {} kbit(s)", self.package.len() / 1024);
        }
    }
}
