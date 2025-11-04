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

build.getOutput = function(exe, args, config)
    if config == nil then config = {} end
    config.exe = exe
    config.args = args
    return bp3d.build.command.output(config)
end

build.render = function(template, args)
    for k, v in pairs(args) do
        bp3d.util.utf8.replace(template, "{" .. k .. "}", v)
    end
    return template
end

return build
