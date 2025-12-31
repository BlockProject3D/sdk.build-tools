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

local Packager = require "bp3d.packager"
local context = require "bp3d.util.context"
local build = require "bp3d.util.build"
local artifact = require "bp3d.util.artifact"
local templates = require "bp3d.templates.framework"

local Framework = Class(Packager)

Framework.argTypes = {
    name = { type = "string" },
    identifier = { type = "string" },
    umbrella = { type = "string", optional = true }
}

function Framework:buildTarget(ctx)
    local files = baseBuild(ctx)
    print("Adding version information...")
    build.runCargo("rustc", ctx, {
        "--",
        "-Clink-arg=-compatibility_version" .. self.context.package.version,
        "-Clink-arg=-current_version" .. self.context.package.version
    })
    return files
end

function Framework:packageTarget(ctx, artifacts)
    print("Packaging " .. self.args.name .. "-" .. self.context.package.version .. " for target " .. ctx.target)
    local targetPath = context.getTargetPath(ctx)
    local frameworkDir = self.args.name .. ".framework"
    local binDir = nil
    local resDir = nil
    local moduleDir = nil
    local isDarwin = bp3d.util.utf8.contains(ctx.target, "darwin")
    if isDarwin then
        binDir = frameworkDir .. "/Versions/A/"
        resDir = frameworkDir .. "/Versions/A/Resources"
        moduleDir = frameworkDir .. "/Versions/A/Modules"
    else
        binDir = frameworkDir
        resDir = frameworkDir
        moduleDir = frameworkDir .. "/Modules"
    end
    frameworkDir = targetPath:join(frameworkDir)
    binDir = targetPath:join(binDir)
    resDir = targetPath:join(resDir)
    moduleDir = targetPath:join(moduleDir)
    print("Cleaning directories...")
    build.clean(frameworkDir, binDir, resDir, moduleDir)
    print("Generating frameowrk " .. tostring(frameworkDir) .. "...")
    build.run("lipo", {
        "-create",
        artifact.findFirst(artifacts, "lib::dynamic"):path(),
        "-output",
        binDir:join(self.args.name)
    })
    build.run("install_name_tool", {
        "-id",
        "@rpath/" .. self.args.name .. ".framework/" .. self.args.name,
        self.args.name
    }, { workdir = binDir })
    if isDarwin then
        bp3d.files.symlink("A", frameworkDir:join("Versions/Current"))
        bp3d.files.symlink("Versions/Current/" .. self.args.name, frameworkDir:join(self.args.name))
        bp3d.files.symlink("Versions/Current/Resources", frameworkDir:join("Resources"))
        bp3d.files.symlink("Versions/Current/Modules", frameworkDir:join("Modules"))
    end
    local includes = artifact.find(artifacts, "header")
    if bp3d.util.table.count(includes) > 0 then
        print("Adding headers...")
        local headerPath = binDir:join("Headers")
        for _, include in pairs(includes) do
            local name = include:name()
            if bp3d.util.string.startsWith(name, self.args.name) then
                --stupidly broken lua language which requires a +2 where it does not make any sense...
                name = name:sub(#self.args.name + 2)
            end
            bp3d.files.copyFile(include:path(), headerPath:join(name))
        end
        if isDarwin then
            bp3d.files.symlink("Versions/Current/Headers", frameworkDir:join("Headers"))
        end
        local umbrella = self.args.umbrella or self.args.name .. ".h"
        local umbrellaPath = headerPath:join(umbrella)
        if not bp3d.files.exists(umbrellaPath) then
            bp3d.files.writeText(umbrellaPath,
                "/* Empty generated umbrella header to ensure Xcode can link the framework. */")
        end
        bp3d.files.writeText(moduleDir:join("module.modulemap"), build.render(templates.MODULE_MAP_TEMPLATE, {
            NAME = self.args.name,
            UMBRELLA = umbrella
        }))
    end
    print("Generating Info.plist...")
    local buildNumber = bp3d.util.utf8.replace(build.getOutput("sw_vers", { "-buildVersion" }), "\n", "")
    local platforms
    if isDarwin then
        platforms = "<string>MacOSX</string>"
    else
        platforms = "<string>iPhoneOS</string>\n        <string>iPadOS</string>"
    end
    bp3d.files.writeText(resDir:join("Info.plist"), build.render(templates.PLIST, {
        NAME = self.args.name,
        VERSION = self.context.package.version,
        BUILD_NUMBER = buildNumber,
        IDENTIFIER = self.args.identifier,
        PLATFORMS = platforms
    }))
end

function Framework:package()
    local out = self.context.path:join("target/" .. self.args.name .. ".xcframework");
    print("Generating XC framework " .. tostring(out) .. "...")
    build.clean(out)
    local args = { "xcodebuild", "-create-xcframework" }
    for _, target in ipairs(self.context.targets) do
        table.insert(args, "-framework")
        table.insert(args, context.getTargetPath(self.context, target):join(self.args.name .. ".framework"))
    end
    table.insert(args, "-output")
    table.insert(args, out)
    build.run("xcrun", args)
end

return Framework
