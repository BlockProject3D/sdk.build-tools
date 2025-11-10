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
local build = require "bp3d.util.build"

local Debug = Class(Packager)

function Debug:buildTarget(ctx)
    local files = bp3d.files.list(ctx.path:join("target"):join(ctx.configuration))
    local bins = {}
    for _, v in ipairs(files) do
        if v.type == "file" then
            local output = build.getOutput("file", { v.path })
            if bp3d.util.utf8.contains(output, "Mach-O") and bp3d.util.utf8.contains(output, "executable") then
                bins[v.name] = v.path
            end
        end
    end
    for k, v in pairs(bins) do
        print("Applying codesign to executable name " .. k .. "...")
        build.run("codesign", { "-s", "-", "-v", "-f", "--entitlements", bp3d.build.files.getResourcesPath():join("entitlements.xml"), v })
    end
    return bp3d.build.List.new()
end

return Debug
