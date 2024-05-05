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

use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::process::Command;
use mlua::{FromLua, Lua, Table, Value};
use crate::lua::engine::Lib;

struct CommandInfo<'lua> {
    pub exe: mlua::String<'lua>,
    pub args: Option<Vec<mlua::String<'lua>>>,
    pub env: Option<HashMap<mlua::String<'lua>, mlua::String<'lua>>>,
    pub working_directory: Option<mlua::String<'lua>>
}

impl<'lua> CommandInfo<'lua> {
    pub fn into_command(self) -> Command {
        let mut cmd = Command::new(as_os_str(&self.exe));
        if let Some(args) = self.args {
            cmd.args(args.iter().map(|v| as_os_str(v)));
        }
        if let Some(env) = self.env {
            cmd.envs(env.iter().map(|(k, v)| (as_os_str(k), as_os_str(v))));
        }
        if let Some(workdir) = self.working_directory {
            cmd.current_dir(as_os_str(&workdir));
        }
        cmd
    }
}

impl<'lua> FromLua<'lua> for CommandInfo<'lua> {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let table = Table::from_lua(value, lua)?;
        Ok(CommandInfo {
            exe: FromLua::from_lua(table.get("exe")?, lua)?,
            args: FromLua::from_lua(table.get("args")?, lua)?,
            env: FromLua::from_lua(table.get("env")?, lua)?,
            working_directory: FromLua::from_lua(table.get("workingDirectory")?, lua)?
        })
    }
}

fn as_os_str<'a>(bytes: &'a mlua::String) -> Cow<'a, OsStr> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        OsStr::from_bytes(bytes.as_bytes()).into()
    }
    #[cfg(windows)]
    {

    }
}

pub struct CommandLib;

fn command_run(_: &Lua, info: CommandInfo) -> mlua::Result<(bool, Option<i32>)> {
    let mut cmd = info.into_command();
    let status = cmd.status()?;
    Ok((status.success(), status.code()))
}

fn command_output<'lua>(lua: &'lua Lua, info: CommandInfo) -> mlua::Result<mlua::String<'lua>> {
    let mut cmd = info.into_command();
    let output = cmd.output()?;
    lua.create_string(output.stdout)
}

impl Lib for CommandLib {
    const NAME: &'static str = "command";

    fn load(&self, lua: &Lua, table: &Table) -> mlua::Result<()> {
        table.set("run", lua.create_function(command_run)?)?;
        table.set("output", lua.create_function(command_output)?)?;
        Ok(())
    }
}
