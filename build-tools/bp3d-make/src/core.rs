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

use bp3d_sdk_util::ResultExt;
use crate::builder::interface::{Builder, Context, Module, OutputList};
use crate::model::Workspace;

pub fn run_builder<B: Builder>(context: &Context, module: &Module, outputs: &mut OutputList) {
    println!("Configuring module {} using builder {}...", module.name, B::NAME);
    let builder = B::do_configure(context, module).expect_exit("Failed to configure module", 1);
    println!("Compiling module {}...", module.name);
    builder.do_compile(context, module).expect_exit("Failed to compile module", 1);
    println!("Adding output files...");
    builder.list_outputs(context, module, outputs).expect_exit("Failed to get module outputs", 1);
}

pub fn run_workspace(context: &Context) {
    let data = std::fs::read_to_string(context.root.join("bp3d-make.toml")).expect_exit("Failed to load workspace configuration", 1);
    let workspace: Workspace = toml::from_str(&data).expect_exit("Failed to read workspace configuration", 1);
    let mut outputs = OutputList::new();
    for (name, member) in workspace.modules {
        let path = context.root.join(member.path.as_deref().unwrap_or(&name));
        let module = Module {
            name: &name,
            path: &path
        };
        member.ty.call(context, &module, &mut outputs);
    }
}
