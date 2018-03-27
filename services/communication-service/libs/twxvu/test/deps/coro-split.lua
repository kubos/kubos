--[[lit-meta
  name = "creationix/coro-split"
  version = "2.0.0"
  homepage = "https://github.com/luvit/lit/blob/master/deps/coro-split.lua"
  description = "An coro style helper for running tasks concurrently."
  tags = {"coro", "split"}
  license = "MIT"
  author = { name = "Tim Caswell" }
]]

-- Split takes several functions as input and runs them in concurrent coroutines.
-- The split function will itself block and wait for all three to finish.
-- The results of the functions will be returned from split.

return function (...)
  local tasks = {...}
  for i = 1, #tasks do
    assert(type(tasks[i]) == "function", "all tasks must be functions")
  end
  local thread = coroutine.running()
  local left = #tasks
  local results = {}
  local function check()
    left = left - 1
    if left == 0 then
      assert(coroutine.resume(thread, unpack(results)))
    end
  end
  for i = 1, #tasks do
    coroutine.wrap(function ()
      results[i] = tasks[i]()
      check()
    end)()
  end
  return coroutine.yield()
end
