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

-- This transport acts as a transparent UDP bridge for real UDP clients and
-- services.

local uv = require 'uv'
local make_callback = require 'make-callback'
local wrapper = require 'wrapper'

-- config.expose-ports - A list of ports to expose locally for forwarding.
-- config.expose-host - Optionally bind exposed ports to a specific network
--                      interface by specifying the ip address.
return function (config)
  -- The framework will populate this with io.send(frame)
  local io = {}

  local handles = {}

  local function get_handle(remote_port, host, port)
    local handle = handles[remote_port]
    if handle then return handle end
    local pressure = 0

    local on_recv = wrapper(function (err, data, addr)
      assert(not err, err)
      if not data then return end

      pressure = pressure + 1
      if pressure == 2 then
        print("PAUSE CLIENT")
        handle:send('\x01', addr.ip, addr.port)
      end

      io.send {
        dest = remote_port,
        source = addr.port,
        data = data
      }

      pressure = pressure - 1
      if pressure == 1 then
        print("RESUME CLIENT")
        handle:send('\x02', addr.ip, addr.port)
      end

    end)

    handle = uv.new_udp()
    assert(handle:bind(host or '127.0.0.1', port or 0))
    assert(handle:recv_start(on_recv))
    -- p("New handle", handle:getsockname())

    handles[remote_port] = handle
    return handle
  end

  local expose = config['expose-ports']
  if expose then
    local host = config['expose-host'] or '127.0.0.1'
    for i = 1, #expose do
      local port = expose[i]
      local handle = get_handle(port, host, port)
      p("Exposing port", handle:getsockname())
    end
  end

  function io.receive(packet)
    if not packet then return end
    local handle = get_handle(packet.source)
    handle:send(packet.data, "127.0.0.1", packet.dest, make_callback())
    return coroutine.yield()
  end

  return io
end
