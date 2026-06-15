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

local build = {}

build.clean = function(...)
    for _, v in ipairs({ ... }) do
        if bp3d.files.exists(v) then
            bp3d.files.deleteDir(v)
        end
        bp3d.files.createDir(v)
    end
end

build.run = function(exe, args, config)
    if config == nil then config = {} end
    config.exe = exe
    config.args = args
    local success, code = bp3d.build.command.run(config)
    assert(success and code == 0, "command failed")
end

build.spawn = function(exe, args, eventThread, userdata, config)
    if config == nil then config = {} end
    config.exe = exe
    config.args = args
    local co = coroutine.create(eventThread)
    coroutine.resume(co, userdata)
    local success, code = bp3d.build.command.spawn(config, co)
    assert(success and code == 0, "command failed")
end

build.getOutput = function(exe, args, config)
    if config == nil then config = {} end
    config.exe = exe
    config.args = args
    return bp3d.build.command.output(config)
end

build.render = function(template, args)
    for k, v in pairs(args) do
        template = bp3d.util.utf8.replace(template, "{" .. k .. "}", v)
    end
    return template
end

build.getCargo = function(cmd, ctx, args2)
    if not bp3d.files.exists(ctx.path:join("Cargo.toml")) then
        return nil
    end
    local args = {
        cmd
    }
    if ctx.target then
        table.insert(args, "--target")
        table.insert(args, ctx.target)
    end
    if ctx.configuration == "release" then
        table.insert(args, "--release")
    end
    if ctx.features then
        if #ctx.features > 0 then
            table.insert(args, "--features")
            for _, v in ipairs(ctx.features) do
                table.insert(args, v)
            end
        end
    else
        table.insert(args, "--all-features")
    end
    for _, v in ipairs(args2) do
        table.insert(args, v)
    end
    return args, { workdir = ctx.path }
end

build.runCargo = function(cmd, ctx, args2, env)
    local args, config = build.getCargo(cmd, ctx, args2)
    if not args then
        return
    end
    config.env = env
    build.run("cargo", args, config)
end

build.runBP3D = function(subPath, cmd, ctx, args)
    local exe = bp3d.build.files.getExecutablePath()
    local args1 = { cmd }
    if ctx.configuration then
        bp3d.util.table.concat(args1, { "-c", ctx.configuration })
    end
    if ctx.targets then
        for _, v in ipairs(ctx.targets) do
            bp3d.util.table.concat(args1, { "-t", v })
        end
    elseif ctx.target then
        bp3d.util.table.concat(args1, { "-t", ctx.target })
    end
    if ctx.features then
        for _, v in ipairs(ctx.features) do
            bp3d.util.table.concat(args1, { "-f", v })
        end
    else
        table.insert(args1, "-a")
    end
    bp3d.util.table.concat(args1, args)
    build.run(exe, args1, { workdir = ctx.path:join(subPath) })
end

return build
