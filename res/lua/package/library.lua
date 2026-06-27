-- Copyright (c) 2026, BlockProject 3D
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

local UnixDist = require "bp3d.package.unix-dist"
local artifact = require "bp3d.util.artifact"
local context = require "bp3d.util.context"
local build = require "bp3d.util.build"
local templates = require "bp3d.templates.library"

local Library = Class(UnixDist)

Library.argTypes = {
    name = { type = "string" }
}

function Library:buildTarget(ctx)
    local files = baseBuild(ctx)
    if bp3d.util.string.contains(ctx.target, "apple") then
        print("Adding version information...")
        build.runCargo("rustc", ctx, {
            "--",
            "-Clink-arg=-compatibility_version" .. self.context.package.version,
            "-Clink-arg=-current_version" .. self.context.package.version
        })
    elseif bp3d.util.string.contains(ctx.target, "msvc") then
        build.runCargo("rustc", ctx, {
            "--",
            "--emit",
            "link=target/" .. ctx.target .. "/" .. ctx.configuration .. "/" .. self.args.name .. ".dll"
        })
        --TODO: Inject product information RC file.
    end
    return files
end

function Library:packageTarget(ctx, artifacts)
    UnixDist.packageTarget(self, ctx, artifacts)
    local coreLibName = artifact.findFirst(artifacts, "lib::dynamic"):path():name()
    local targetPath = context.getTargetPath(ctx)
    local distPath = targetPath:join("dist")
    local libName = ""
    if not bp3d.util.string.contains(ctx.target, "msvc") then
        local originalLibPath = distPath:join("lib"):join(coreLibName)
        libName = "lib" .. self.args.name .. "." .. originalLibPath:extension()
        local newLibPath = distPath:join("lib"):join(libName)
        bp3d.files.rename(originalLibPath, newLibPath)
    else
        libName = self.args.name .. ".dll"
        local staticLibPath = targetPath:join(self.args.name .. ".lib")
        bp3d.files.copyFile(staticLibPath, distPath:join("lib"):join(self.args.name .. ".lib"))
        bp3d.files.copyFile(targetPath:join(libName), distPath:join("lib"):join(libName))
    end
    if bp3d.util.string.contains(ctx.target, "apple") then
        build.run("install_name_tool", {
            "-id",
            "@rpath/" .. libName,
            libName
        }, { workdir = distPath:join("lib") })
    elseif bp3d.util.string.contains(ctx.target, "msvc") then
        -- Nothing to do as cargo rustc has already done it...
    else
        build.run("patchelf", {
            "--set-soname",
            libName,
            libName
        }, { workdir = distPath:join("lib") })
    end
    local cmakePath = distPath:join("usr"):join("Find" .. self.args.name .. ".cmake")
    bp3d.files.writeText(cmakePath, build.render(templates.CMAKE, {
        NAME = self.args.name,
        LIB_NAME = libName
    }))
end

return Library
