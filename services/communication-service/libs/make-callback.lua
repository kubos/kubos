return function ()
  local thread = coroutine.running()
  return function (err, value, ...)
    if err then
      assert(coroutine.resume(thread, nil, err))
    else
      assert(coroutine.resume(thread, value == nil and true or value, ...))
    end
  end
end
