--[[
Copyright (C) 2018 Kubos Corporation
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
  http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
]]

-- Fancy version of `coroutine.wrap` that also catches errors and logs them to
-- stdout with full stack trace.  Also the wrapped function is reusable, so good
-- for event handlers.
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
