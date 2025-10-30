local args = {}

args.create = function(typeInfo)
    local tbl = {
        __types = typeInfo
    }
    for k, v in pairs(typeInfo) do
        if v.optional then
            tbl[k] = v.default
        elseif v.type == "string" then
            tbl[k] = ""
        elseif v.type == "number" then
            tbl[k] = 0
        elseif v.type == "boolean" then
            tbl[k] = false
        elseif v.type == "enum" then
            tbl[k] = v.enum[1]
        end
    end
    return tbl
end

args.update = function(tbl, args)
    assert(tbl ~= nil, "no argument type map passed")
    for k, v in pairs(tbl.__types) do
        if not v.optional then
            assert(args ~= nil, "arguments table is nil")
            if v.type == "enum" then
                assert(type(args[k]) == "string", "invalid type for argument " .. k)
                assert(bp3d.util.table.contains(v.enum, args[k]), "invalid enum variant " .. args[k])
            else
                assert(type(args[k]) == v.type, "invalid type for argument " .. k)
            end
            tbl[k] = args[k]
        elseif args ~= nil and args[k] ~= nil then
            if v.type == "enum" then
                assert(type(args[k]) == "string", "invalid type for argument " .. k)
                assert(bp3d.util.table.contains(v.enum, args[k]), "invalid enum variant " .. args[k])
            else
                assert(type(args[k]) == v.type, "invalid type for argument " .. k)
            end
            tbl[k] = args[k]
        end
    end
end

return args
