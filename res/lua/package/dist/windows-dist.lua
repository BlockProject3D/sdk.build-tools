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

local BaseDist = require "bp3d.package.dist.base"
local context = require "bp3d.util.context"
local artifact = require "bp3d.util.artifact"
local build = require "bp3d.util.build"

local WindowsDist = Class(BaseDist)

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

function WindowsDist:buildTarget(ctx)
    local artifacts = baseBuild(ctx)
    local extPath = BaseDist.getExtPath(ctx)
    if not bp3d.files.exists(extPath) then return artifacts end
    local bin = extPath:join("bin")
    local lib = extPath:join("lib")
    if bp3d.files.exists(bin) then
        BaseDist.appendObjects(artifacts, bin, function(path, name) return bp3d.build.Artifact.findBin(path:parent(), name) end)
        BaseDist.appendObjects(artifacts, bin, function(path, name) return bp3d.build.Artifact.findLib(path:parent(), name, "dynamic") end)
    end
    if bp3d.files.exists(lib) then
        appendObjects(artifacts, lib,
            function(path, name) return bp3d.build.Artifact.findLib(path:parent(), name, "static") end)
    end
    BaseDist.addExtUsr(ctx, artifacts)
    return artifacts
end

function WindowsDist:packageTarget(ctx, artifacts)
    local targetPath = context.getTargetPath(ctx)
    local distPath = BaseDist.getDistPath(ctx)

    BaseDist.packUsr(ctx, artifacts)

    -- Package binarries.
    local bins = artifact.find(artifacts, "bin")
    local libs = artifact.findDynamicLibraries(artifacts, targetPath)
    if bp3d.util.table.count(bins) > 0 or bp3d.util.table.count(libs) > 0 then
        print("Packaging binarries...")
        local binDir = distPath:join("bin")
        build.clean(binDir)
        for _, v in pairs(bins) do
            artifact.copyTo(v, binDir)
        end
        for _, v in pairs(libs) do
            artifact.copyTo(v, binDir)
        end
    end

    -- Package libraries.
    local libs = artifact.find(artifacts, "lib::static")
    local libDir = distPath:join("lib")
    if bp3d.util.table.count(libs) > 0 then
        print("Packaging libraries...")
        build.clean(libDir)
        for _, v in pairs(libs) do
            artifact.copyTo(v, libDir)
        end
    end
end

return WindowsDist
