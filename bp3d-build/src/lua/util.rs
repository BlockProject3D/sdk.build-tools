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

use crate::system::Package;
use bp3d_lua::vm::table::Table;
use bp3d_lua::vm::Result;
use bp3d_lua::vm::Vm;

pub fn convert_package<'a>(vm: &'a Vm, package: &dyn Package) -> Result<Table<'a>> {
    let mut res = Table::with_capacity(vm, 0, 3);
    res.set(c"name", package.get_primary_name())?;
    res.set(c"version", package.get_primary_version())?;
    if package.get_components() > 0 {
        let mut components = Table::with_capacity(vm, 0, package.get_components());
        for i in 0..package.get_components() {
            let mut component = Table::with_capacity(vm, 0, 3);
            let c = package.get_component(i);
            component.set("name", c.get_name())?;
            component.set("version", c.get_version())?;
            component.set("description", c.get_description())?;
            component.set("public", c.is_public())?;
            components.set(c.get_short_name(), component)?;
        }
        res.set("components", components)?;
    }
    if !package.features().is_empty() {
        let mut features = Table::with_capacity(vm, package.features().len(), 0);
        for feature in package.features() {
            features.push(&**feature)?;
        }
        res.set("features", features)?;
    }
    Ok(res)
}
