local Packager = require "bp3d.packager"
local build = require "bp3d.util.build"

local Debug = Class(Packager)

function Debug:buildTarget(ctx)
    local files = bp3d.build.files.list(ctx.path:join("target"):join(ctx.configuration))
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
        build.run("codesign", { "-s", "-", "-v", "-f", "--entitlements", bp3d.build.files.getSharePath():join("entitlements.xml"), v })
    end
    return bp3d.build.List.new()
end

return Debug
