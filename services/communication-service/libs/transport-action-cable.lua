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

-- This transport connects to Major Tom via action-cable and an authentication
-- token using outbound websockets.

-- When using this tranport, openssl is required so you'll need to use
-- `luvi-regular` instead of `luvi-tiny`.

local uv = require 'uv'
local ws = require 'coro-websocket'
local wrap = require('action-frame').wrap
local subscribe = require('action-frame').subscribe
local make_callback = require 'make-callback'

-- config.url - The websocket url to connect to. (For example ws://localhost/...)
-- config.token - The authentication JWT in base64/url form.
return function (config)
  -- The framework will populate this with io.send(frame)
  local io = {}

  local url = assert(config.url, 'Missing URL')
  local token = assert(config.token, 'Missing token')
  local options = ws.parseUrl(url)

  p("action-cable", options)
  options.pathname = options.pathname .. '?agent_token=' .. token

  -- TODO use server cert instead of insecure flag
  options.tls = { insecure = true }
  options.headers = {{"User-Agent", "kubos-communication-service"}}

  local function connect()
    local timeout = 100
    local res, read, write
    while true do
      res, read, write = ws.connect(options)
      if res then break end
      print(read, "Trying again in " .. timeout .. 'ms')
      local timer = uv.new_timer()
      timer:start(timeout, 0, make_callback())
      coroutine.yield()
      timer:close()
      timeout = timeout * 2
    end
    assert(res.code == 101)

    write {
      opcode = 1,
      payload = subscribe
    }

    read, write = wrap(read, write)

    io.receive = write

    coroutine.wrap(function ()
      for frame in read do
        io.send(frame)
      end
      connect()
    end)()
  end

  connect()

  return io
end
