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

local uv = require 'uv'
local cbor_message_protocol = require 'cbor-message-protocol'

return function (make_service, port)

  local send_message
  local server = uv.new_udp()
  assert(server:bind('127.0.0.1', port))

  local meta = {}
  function meta:__index(key)
    local fn = function (...)
      send_message({ self.id, key, ... }, self.ip, self.port)
    end
    self[key] = fn
    return fn
  end

  local channels = {}
  local function get_channel(id, addr)
    local channel = channels[id]
    if channel then
      channel.ip = addr.ip
      channel.port = addr.port
    else
      channel = setmetatable({
        id = id,
        ip = addr.ip,
        port = addr.port,
       }, meta)
      channel.service = make_service(channel)
      channels[id] = channel
    end
    return channel
  end

  local function on_message(message, addr)
    local channel
    local success, error = xpcall(function ()
      assert(type(message) == 'table' and #message >= 1, 'Message must be list')
      local id = table.remove(message, 1)
      channel = get_channel(id, addr)
      local fn = channel.service[table.remove(message, 1)]
      assert(type(fn) == 'function', 'Invalid command')
      fn(unpack(message))
    end, debug.traceback)
    if not success then
      print(error)
      if channel then
        channel.error(error)
      end
    end
  end

  send_message = cbor_message_protocol(server, on_message, true)

  p('Shell Service: UDP server bound', server:getsockname())

end
