return function (fn)
  return function (...)
    local args = { ... }
    return coroutine.wrap(function()
      local success, result = xpcall(function ()
        return fn(unpack(args))
      end, debug.traceback)
      if not success then
        print(result)
      end
    end)()
  end
end
