local context = {}

context.getTargetPath = function(ctx, target)
    if target == nil then target = ctx.target end
    return ctx.path:join("target"):join(target):join(ctx.configuration)
end

return context
