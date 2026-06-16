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
local artifact = require "bp3d.util.artifact"
local build = require "bp3d.util.build"

local UnixDist = Class(Packager)

local function appendObjects(artifacts, path, f)
    local files = bp3d.files.list(path)
    for _, v in ipairs(files) do
        if v.type == "file" then
            local name = v.path:name()
            assert(name ~= nil)
            artifacts:add(f(v.path, name))
        end
    end
end

function UnixDist:buildTarget(ctx)
    local artifacts = baseBuild(ctx)
    local extPath = ctx.path:join("target"):join(ctx.target):join("ext")
    if not bp3d.files.exists(extPath) then return artifacts end
    local usrExtPath = extPath:join("usr")
    local bin = extPath:join("bin")
    local lib = extPath:join("lib")
    local include = usrExtPath:join("include")
    local config = usrExtPath:join("etc")
    local res = usrExtPath:join("share")
    if bp3d.files.exists(bin) then
        appendObjects(artifacts, bin, function(path, name) return bp3d.build.Artifact.findBin(path:parent(), name) end)
    end
    if bp3d.files.exists(lib) then
        appendObjects(artifacts, lib, function(path, name) return bp3d.build.Artifact.findLib(path:parent(), name, "dynamic") end)
        appendObjects(artifacts, lib, function(path, name) return bp3d.build.Artifact.findLib(path:parent(), name, "static") end)
    end
    if bp3d.files.exists(include) then
        artifacts:addFolder("header", include, "")
    end
    if bp3d.files.exists(res) then
        artifacts:addFolder("other", res, "")
    end
    if bp3d.files.exists(config) then
        artifacts:addFolder("config", config, "")
    end
    return artifacts
end

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
            bp3d.files.copyFile(v:path(), binDir:join(v:name()))
        end
    end

    -- Package libraries.
    local libs = artifact.findDynamicLibraries(artifacts, targetPath)
    local libDir = distPath:join("lib")
    if bp3d.util.table.count(libs) > 0 then
        print("Packaging libraries...")
        build.clean(libDir)
        for _, v in pairs(libs) do
            bp3d.files.copyFile(v.path, libDir:join(v.name))
        end
    end

    -- Package headers.
    local headers = artifact.find(artifacts, "header")
    if bp3d.util.table.count(headers) > 0 then
        print("Packaging headers...")
        local includeDir = usrPath:join("include")
        build.clean(includeDir)
        for _, v in pairs(headers) do
            bp3d.files.copyFile(v:path(), includeDir:join(v:name()))
        end
    end

    -- Package configs.
    local configs = artifact.find(artifacts, "config")
    if bp3d.util.table.count(configs) > 0 then
        print("Packaging configs...")
        local etcDir = distPath:join("etc")
        build.clean(etcDir)
        for _, v in pairs(configs) do
            bp3d.files.copyFile(v:path(), etcDir:join(v:name()))
        end
    end

    -- Package resources.
    local resources = artifact.find(artifacts, "resource")
    if bp3d.util.table.count(resources) > 0 then
        print("Packaging resources...")
        local shareDir = usrPath:join("share")
        build.clean(shareDir)
        for _, v in pairs(resources) do
            bp3d.files.copyFile(v:path(), shareDir:join(v:name()))
        end
    end
end

return UnixDist
