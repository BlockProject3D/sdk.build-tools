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

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use bp3d_lua::decl_lib_func;
use bp3d_lua::libs::files::{SandboxPath, SandboxPathBuf};
use bp3d_lua::libs::Lib;
use bp3d_lua::util::Namespace;
use bp3d_lua::vm::function::types::RFunction;
use bp3d_lua::vm::table::Table;
use bp3d_lua::vm::Vm;
use bp3d_util::simple_error;

simple_error! {
    pub Error {
        Lua(bp3d_lua::vm::error::Error) => "lua error: {}",
        Io(std::io::Error) => "io error: {}"
    }
}

struct CommandInfo {
    pub exe: SandboxPathBuf,
    pub args: Option<Vec<SandboxPathBuf>>,
    pub env: Option<HashMap<String, String>>,
    pub workdir: Option<PathBuf>
}

impl CommandInfo {
    pub fn from_table(vm: &Vm, table: &Table) -> bp3d_lua::vm::Result<Self> {
        let workdir: Option<SandboxPath> = table.get(c"workdir")?;
        let workdir = workdir.map(|v| v.to_path(vm).ok().map(PathBuf::from)).flatten();
        Ok(CommandInfo {
            exe: table.get(c"exe")?,
            args: table.get(c"args")?,
            env: table.get(c"env")?,
            workdir,
        })
    }
}

impl CommandInfo {
    pub fn into_command(self) -> Command {
        println!("{:?}", self.exe.as_os_str());
        println!("{:?}", self.workdir);
        let mut cmd = Command::new(self.exe.as_os_str());
        if let Some(args) = self.args {
            cmd.args(args.iter().map(|v| v.as_os_str()));
        }
        if let Some(env) = self.env {
            cmd.envs(env.iter().map(|(k, v)| (k, v)));
        }
        if let Some(workdir) = self.workdir {
            cmd.current_dir(&workdir);
        }
        cmd
    }
}

decl_lib_func! {
    fn command_run(vm: &Vm, table: Table) -> Result<(bool, Option<i32>), Error> {
        let info = CommandInfo::from_table(vm, &table).map_err(Error::Lua)?;
        let mut cmd = info.into_command();
        let status = cmd.status().map_err(Error::Io)?;
        Ok((status.success(), status.code()))
    }
}

decl_lib_func! {
    fn command_output(vm: &Vm, table: Table) -> Result<String, Error> {
        let info = CommandInfo::from_table(vm, &table).map_err(Error::Lua)?;
        let mut cmd = info.into_command();
        let output = cmd.output().map_err(Error::Io)?;
        Ok(String::from_utf8_lossy(&output.stdout).into())
    }
}

pub struct CommandLib;

impl Lib for CommandLib {
    const NAMESPACE: &'static str = "bp3d.build.command";

    fn load(&self, namespace: &mut Namespace) -> bp3d_lua::vm::Result<()> {
        namespace.add([
            ("run", RFunction::wrap(command_run)),
            ("output", RFunction::wrap(command_output))
        ])
    }
}
