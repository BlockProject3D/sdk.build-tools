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
local Script = require "bp3d.script"

local ReleaseInfo = Class(Script)

function ReleaseInfo:canRelease(dirName, component)
    return component.public
end

function ReleaseInfo:getParameters()
    local package = self.context.package
    local components = package.components
    if components == nil then
        components = { [package.name] = { name = package.name, version = package.version, public = true } }
    end
    local tag = ""
    local name = nil
    local version = nil
    local compList = "## Components:\n\n"
    for k, v in pairs(components) do
        if self:canRelease(k, v) then
            tag = tag .. k .. "-" .. v.version .. "+"
            name = v.name
            version = v.version
            compList = compList .. "    - " .. v.name .. " (" .. v.version .. ") [new]\n"
        else
            compList = compList .. "    - " .. v.name .. " (" .. v.version .. ")\n"
        end
    end
    if #tag <= 1 then
        print("This package does not have any new release available")
        return nil
    end
    tag = tag:sub(0, #tag - 1)
    return {
        TAG = tag,
        NAME = name .. " release " .. version,
        COMPONENTS = compList
    }
end

function ReleaseInfo:checkTagExists(tag)
    local function eventHandler()
        repeat
            local ty, _ = coroutine.yield()
        until ty == nil
    end
    local success, status = build.spawn("git", { "rev-parse", tag }, eventHandler, nil,
        { ignoreFailure = true })
    return success and status == 0
end

function ReleaseInfo:run()
    local params = self:getParameters()
    if params == nil then
        return 1
    end
    if self:checkTagExists(params.TAG) then
        print("Tag already exists, aborting...")
        return 1
    end
    local text = ""
    for name, content in pairs(params) do
        local data = bp3d.util.utf8.replace(content, "\\", "\\\\")
        data = bp3d.util.utf8.replace(data, "\n", "\\n")
        data = bp3d.util.utf8.replace(data, "\r", "\\r")
        data = bp3d.util.utf8.replace(data, "\t", "\\t")
        data = bp3d.util.utf8.replace(data, "'", "\\'")
        text = text .. name .. "='" .. data .. "'\n"
    end
    local out = self.context.path:join("target/release-info.env");
    bp3d.files.writeText(out, text)
end

return ReleaseInfo
