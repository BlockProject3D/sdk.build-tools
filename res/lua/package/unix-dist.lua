local Packager = require "bp3d.packager"
local context = require "bp3d.util.context"
local artifact = require "bp3d.util.artifact"
local build = require "bp3d.util.build"

local UnixDist = Class(Packager)

function UnixDist:packageTarget(ctx, artifacts)
    local targetPath = context.getTargetPath(ctx)
    local distPath = targetPath:join("dist")
    local usrPath = distPath:join("usr")
    build.clean(distPath, usrPath)

    -- Package binarries.
    local bins = artifact.find(artifacts, "bin")
    if bp3d.util.table.count(bins) > 0 then
        print("Packaging binarries...")
        local binDir = distPath:join("bin")
        build.clean(binDir)
        for _, v in pairs(bins) do
            bp3d.build.files.copy(v:path(), binDir:join(v:name()))
        end
    end

    -- Package libraries.
    local libs = artifact.find(artifacts, "lib::dynamic")
    local libDir = distPath:join("lib")
    if bp3d.util.table.count(libs) > 0 then
        print("Packaging libraries...")
        build.clean(libDir)
        for _, v in pairs(libs) do
            bp3d.build.files.copy(v:path(), libDir:join(v:name()))
        end
    end
    local files = bp3d.build.files.list(targetPath)
    local hasLibs = false
    for _, v in ipairs(files) do
        local ext = v.path:extension()
        if v.type == "file" and (ext == "dylib" or ext == "dll" or ext == "so") then
            if not hasLibs then
                if bp3d.util.table.count(libs) == 0 then
                    print("Packaging libraries...")
                    build.clean(libDir)
                end
                hasLibs = true
            end
            bp3d.build.files.copy(v.path, libDir:join(v.name))
        end
    end

    -- Package headers.
    local headers = artifact.find(artifacts, "header")
    if bp3d.util.table.count(headers) > 0 then
        print("Packaging headers...")
        local includeDir = usrPath:join("include")
        build.clean(includeDir)
        for _, v in pairs(headers) do
            bp3d.build.files.copy(v:path(), includeDir:join(v:name()))
        end
    end

    -- Package configs.
    local configs = artifact.find(artifacts, "config")
    if bp3d.util.table.count(configs) > 0 then
        print("Packaging configs...")
        local etcDir = distPath:join("etc")
        build.clean(etcDir)
        for _, v in pairs(configs) do
            bp3d.build.files.copy(v:path(), etcDir:join(v:name()))
        end
    end

    -- Package resources.
    local resources = artifact.find(artifacts, "resource")
    if bp3d.util.table.count(resources) > 0 then
        print("Packaging resources...")
        local shareDir = usrPath:join("share")
        build.clean(shareDir)
        for _, v in pairs(resources) do
            bp3d.build.files.copy(v:path(), shareDir:join(v:name()))
        end
    end
end

return UnixDist
