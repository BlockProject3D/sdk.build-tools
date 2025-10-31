function Class(parent)
    local class = {}
    class.__index = class
    if parent == nil then return class end
    setmetatable(class, parent)
    return class
end

function New(class, args)
    local obj = {}
    setmetatable(obj, class)
    obj:init(args)
    return obj
end
