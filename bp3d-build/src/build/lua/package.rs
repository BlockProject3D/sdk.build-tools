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

use std::borrow::Cow;
use std::path::Path;
use bp3d_lua::vm::table::Table;
use crate::lua::core::Vm;
use crate::system::{Component, Package};

struct ComponentInfo {
    name: String,
    version: String,
    short_name: String,
    description: Option<String>
}

impl Component for ComponentInfo {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_version(&self) -> &str {
        &self.version
    }

    fn get_short_name(&self) -> &str {
        &self.short_name
    }

    fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

pub struct LuaPackage {
    vm: Vm,
    targets: Vec<Cow<'static, str>>,
    configurations: Vec<Cow<'static, str>>,
    features: Vec<Cow<'static, str>>,
    components: Vec<ComponentInfo>,
    name: String,
    version: String,
}

impl LuaPackage {
    pub fn new(path: &Path) -> bp3d_lua::vm::Result<LuaPackage> {
        let mut vm = Vm::new(path)?;
        let main = path.join("build.lua");
        vm.run(&main)?;
        let mut targets: Vec<Cow<'static, str>> = Vec::new();
        let mut configurations: Vec<Cow<'static, str>> = Vec::new();
        let mut features: Vec<Cow<'static, str>> = Vec::new();
        let mut name: String = String::new();
        let mut version: String = String::new();
        let mut comps = Vec::new();
        vm.with_class(|_, class| {
            let targets1: Vec<String> = class.get(c"targets")?;
            let configurations1: Vec<String> = class.get(c"configurations")?;
            let features1: Vec<String> = class.get(c"features")?;
            targets = targets1.into_iter().map(|v| Cow::Owned(v)).collect();
            configurations = configurations1.into_iter().map(|v| Cow::Owned(v)).collect();
            features = features1.into_iter().map(|v| Cow::Owned(v)).collect();
            name = class.get(c"name")?;
            version = class.get(c"version")?;
            let components: Option<Table> = class.get("components")?;
            if let Some(mut components) = components {
                for (short_name, value) in components.iter() {
                    let tbl: Table = value.get()?;
                    comps.push(ComponentInfo {
                        short_name: short_name.get()?,
                        name: tbl.get("name")?,
                        version: tbl.get("version")?,
                        description: tbl.get("description")?
                    })
                }
            }
            Ok(())
        })?;
        Ok(LuaPackage {
            vm,
            targets,
            configurations,
            features,
            name,
            version,
            components: comps
        })
    }

    pub fn vm(&self) -> &Vm {
        &self.vm
    }
}

impl Package for LuaPackage {
    fn get_primary_name(&self) -> &str {
        &self.name
    }

    fn get_primary_version(&self) -> &str {
        &self.version
    }

    fn get_components(&self) -> usize {
        self.components.len()
    }

    fn get_component(&self, index: usize) -> &dyn Component {
        &self.components[index]
    }

    fn targets(&self) -> &[Cow<'_, str>] {
        &self.targets
    }

    fn configurations(&self) -> &[Cow<'_, str>] {
        &self.configurations
    }

    fn features(&self) -> &[Cow<'_, str>] {
        &self.features
    }
}
