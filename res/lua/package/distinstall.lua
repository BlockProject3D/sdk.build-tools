-- Copyright (c) 2025, BlockProject 3D
--
-- All rights reserved.
--
-- Redistribution and use in source and binary forms, with or without modification,
-- are permitted provided that the following conditions are met:
--
--     * Redistributions of source code must retain the above copyright notice,
--       this list of conditions and the following disclaimer.
--     * Redistributions in binary form must reproduce the above copyright notice,
--       this list of conditions and the following disclaimer in the documentation
--       and/or other materials provided with the distribution.
--     * Neither the name of BlockProject 3D nor the names of its contributors
--       may be used to endorse or promote products derived from this software
--       without specific prior written permission.
--
-- THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
-- "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
-- LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
-- A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
-- CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
-- EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
-- PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
-- PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
-- LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
-- NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
-- SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

local build = require "bp3d.util.build"
local context = require "bp3d.util.context"
local Dist = require "bp3d.package.dist"
local templates = require "bp3d.templates.distinstall"

local DistInstall = Class(Dist)

function DistInstall:packageTarget(ctx, artifacts)
    Dist.packageTarget(self, ctx, artifacts)
    local name = self.context.package.name
    local version = self.context.package.version
    local targetPath = context.getTargetPath(ctx)
    print("Building BPX package...")
    build.run("bpxp", { "-t", ctx.target, "-m", "Name=" .. name, "-m", "Version=" .. version, "-cf", "../dist.bpx", "." }, { workdir = targetPath:join("dist") })
    print("Generating installer...")
    bp3d.files.writeText(targetPath:join("installer.rs"), build.render(templates.INSTALLER_MAIN, {
        NAME = name,
        VERSION = version
    }))
    local installerName = "install-" .. name .. "-" .. version .. "." .. ctx.target
    if bp3d.util.string.contains(ctx.target, "windows") then
        installerName = installerName .. ".exe"
    end
    local libPath = bp3d.build.files.getLibraryPath()
    build.run("rustc", { "-L", libPath, "-linstaller", "--edition=2021", "--crate-type", "bin", "installer.rs", "-O", "-o", installerName }, { workdir = targetPath })
end

return DistInstall
