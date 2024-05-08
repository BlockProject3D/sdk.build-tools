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

use std::path::PathBuf;
use bp3d_sdk_util::ResultExt;
use crate::builder::interface::{Builder, Context, Module, OutputList};
use crate::builder::util::PathExt;
use crate::model::Workspace;

pub fn run_builder<B: Builder>(context: &Context, module: &Module, paths: &mut Vec<PathBuf>) {
    println!("Configuring module {} using builder {}...", module.name, B::NAME);
    let builder = B::do_configure(context, module).expect_exit("Failed to configure module", 1);
    println!("Compiling module {}...", module.name);
    builder.do_compile(context, module).expect_exit("Failed to compile module", 1);
    println!("Adding output files...");
    let outputs = builder.list_outputs(context, module).expect_exit("Failed to get module outputs", 1);
    //TODO: Resolve outputs and fill the path list passed as argument.
}

pub fn run_workspace(context: &Context) {
    let data = std::fs::read_to_string(context.root.join("bp3d-make.toml")).expect_exit("Failed to load workspace configuration", 1);
    let workspace: Workspace = toml::from_str(&data).expect_exit("Failed to read workspace configuration", 1);
    let mut paths = Vec::new();
    for (name, member) in workspace.modules {
        let path = context.root.join(member.path.as_deref().unwrap_or(&name));
        let module = Module {
            name: &name,
            path: &path
        };
        member.ty.call(context, &module, &mut paths);
    }
    println!("Creating output target directory...");
    println!("List of outputs: {:?}", paths);
    let root_target = context.root.join("target").join_option(context.target)
        .join(if context.release { "release" } else { "debug" });
    #[cfg(unix)]
    {
        if !root_target.exists() {
            std::fs::create_dir_all(&root_target).expect_exit("Failed to create root target directory", 1);
        }
        for path in paths {
            if let Some(file_name) = path.file_name() {
                let dst = root_target.join(file_name);
                if !dst.exists() {
                    std::os::unix::fs::symlink(
                        std::fs::canonicalize(path).expect_exit("Failed to get absolute path", 1),
                        dst
                    ).expect_exit("Failed to link target file", 1)
                }
            }
        }
    }
    #[cfg(windows)]
    {
        if root_target.exists() {
            std::fs::remove_dir_all(&root_target).expect_exit("Failed to remove root target directory", 1);
        }
        std::fs::create_dir_all(&root_target).expect_exit("Failed to create root target directory", 1);
        for path in paths {
            if let Some(file_name) = path.file_name() {
                std::fs::copy(path, root_target.join(file_name)).expect_exit("Failed to copy target file", 1);
            }
        }
    }
    //let workspace_content = serde_json::to_string(outputs.as_ref()).expect_exit("Failed to generate workspace content JSON file", 1);
    //std::fs::write(root_target.join("manifest.json"), workspace_content).expect_exit("Failed to write workspace content JSON file", 1);
}
