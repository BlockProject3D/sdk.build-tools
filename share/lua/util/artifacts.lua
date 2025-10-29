local artifacts = {}

artifacts.findFirst = function(artifacts, type)
    for _, v in pairs(artifacts) do
        if v:ty() == type then
            return v
        end
    end
    return nil
end

artifacts.find = function(artifacts, type)
    local res = {}
    for k, v in pairs(artifacts) do
        if v:ty() == type then
            res[k] = v
        end
    end
    return res
end

return artifacts
