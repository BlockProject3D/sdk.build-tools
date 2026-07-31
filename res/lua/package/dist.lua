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
local UnixDist = require "bp3d.package.dist.unix-dist"
local WindowsDist = require "bp3d.package.dist.windows-dist"

local Dist = Class(Packager)

function Dist:init(args1)
    Packager.init(self, args1)
    self.unix = New(UnixDist, args1)
    self.windows = New(WindowsDist, args1)
end

function Dist:init2(ctx)
    Packager.init2(self, ctx)
    self.unix:init2(ctx)
    self.windows:init2(ctx)
end

function Dist:buildTarget(ctx)
    if bp3d.util.string.contains(ctx.target, "windows") then
        return self.windows:buildTarget(ctx)
    else
        return self.unix:buildTarget(ctx)
    end
end

function Dist:packageTarget(ctx, artifacts)
    if bp3d.util.string.contains(ctx.target, "windows") then
        return self.windows:packageTarget(ctx, artifacts)
    else
        return self.unix:packageTarget(ctx, artifacts)
    end
end

return Dist
