require "bp3d.class"
local args = require "bp3d.util.args"

local Packager = Class()

function Packager:init(args1)
    if self.argTypes ~= nil then
        self.args = args.create(self.argTypes)
        args.update(self.args, args1)
    end
end

function Packager:build(ctx) end

function Packager:packageTarget(ctx, artifacts) end

function Packager:package(ctx) end

return Packager
