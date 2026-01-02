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

local artifact = {}

artifact.findFirst = function(artifacts, type)
    for _, v in pairs(artifacts) do
        if v:ty() == type then
            return v
        end
    end
    return nil
end

artifact.find = function(artifacts, type)
    local res = {}
    for k, v in pairs(artifacts) do
        if v:ty() == type then
            res[k] = v
        end
    end
    return res
end

artifact.contains = function(artifacts, name)
    for _, v in pairs(artifacts) do
        if v:name() == name then return true end
    end
    return false
end

artifact.findDynamicLibraries = function(artifacts, targetPath)
    local res = {}
    local libs = artifact.find(artifacts, "lib::dynamic")
    for _, v in pairs(libs) do
        table.insert(res, {
            name = v:path():name(),
            path = v:path()
        })
    end
    local files = bp3d.files.list(targetPath)
    for _, v in ipairs(files) do
        local ext = v.path:extension()
        if v.type == "file" and (ext == "dylib" or ext == "dll" or ext == "so") and not artifact.contains(artifacts, v.name) then
            table.insert(res, {
                name = v.name,
                path = v.path
            })
        end
    end
    return res
end

return artifact
