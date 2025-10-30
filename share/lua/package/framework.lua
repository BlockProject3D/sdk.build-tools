local context = require "bp3d.util.context"
local args = require "bp3d.util.args"
local build = require "bp3d.util.build"
local artifact = require "bp3d.util.artifact"
local templates = require "bp3d.templates.framework"

local Framework = {}

Framework.args = args.create({
    name = { type = "string" },
    identifier = { type = "string" },
    umbrella = { type = "string", optional = true }
})

function Framework:init(args1)
    args.update(self.args, args1)
    self.version = ""
end

function Framework:build(ctx)
    self.version = ctx.package.version
end

function Framework:packageTarget(ctx, artifacts)
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
    local frameworkDir = targetPath:join(frameworkDir)
    local binDir = targetPath:join(binDir)
    local resDir = targetPath:join(resDir)
    local moduleDir = targetPath:join(moduleDir)
    print("Cleaning directories...")
    build.clean(frameworkDir, binDir, resDir, moduleDir)
    print("Generating frameowrk...")
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
        bp3d.build.files.symlink("A", frameworkDir:join("Versions/Current"))
        bp3d.build.files.symlink("Versions/Current/" .. self.args.name, frameworkDir:join(self.args.name))
        bp3d.build.files.symlink("Versions/Current/Resources", frameworkDir:join("Resources"))
        bp3d.build.files.symlink("Versions/Current/Modules", frameworkDir:join("Modules"))
    end
    local includes = artifact.find(artifacts, "header")
    if bp3d.util.table.count(includes) > 0 then
        print("Adding headers...")
        local headerPath = binDir:join("Headers")
        for _, include in pairs(includes) do
            bp3d.build.files.copy(include:path(), headerPath:join(include:name()))
        end
        if isDarwin then
            bp3d.build.files.symlink("Versions/Current/Headers", frameworkDir:join("Headers"))
        end
        local umbrella = self.args.umbrella or self.args.name .. ".h"
        local umbrellaPath = headerPath:join(umbrella)
        if not bp3d.build.files.exists(umbrellaPath) then
            bp3d.build.files.writeText(umbrellaPath,
                "/* Empty generated umbrella header to ensure Xcode can link the framework. */")
        end
        bp3d.build.files.writeText(moduleDir:join("module.modulemap"), build.render(templates.MODULE_MAP_TEMPLATE, {
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
    bp3d.build.files.writeText(resDir:join("Info.plist"), build.render(templates.PLIST, {
        NAME = self.args.name,
        VERSION = self.version,
        BUILD_NUMBER = buildNumber,
        IDENTIFIER = self.args.identifier,
        PLATFORMS = platforms
    }))
end

function Framework:package(ctx)
    print("Generating XC framework...")
    local out = ctx.path:join("target/" .. self.args.name .. ".xcframework");
    build.clean(out)
    local args = { "xcodebuild", "-create-xcframework" }
    for _, target in ipairs(ctx.targets) do
        table.insert(args, "-framework")
        table.insert(args, context.getTargetPath(ctx, target):join(self.args.name .. ".framework"))
    end
    table.insert(args, "-output")
    table.insert(args, out)
    build.run("xcrun", args)
end

return Framework
