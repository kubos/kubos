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
local cbor = require 'cbor'

-- Default lua strings to utf8 strings in cbor encoding.
cbor.type_encoders.string = cbor.type_encoders.utf8string

return function (service, service_port)

  local server = uv.new_udp()
  assert(server:bind('127.0.0.1', service_port))

  -- Map from channel_id/hash to {ip,port}
  local addrs = {}

  local function send(channel_id, ...)
    local ip, port = unpack(assert(addrs[channel_id]))
    local message = {channel_id, ...}
    p('->', message)
    local encoded = cbor.encode(message)
    server:send(encoded, ip, port)
  end

  service.send = send

  coroutine.wrap(function ()
    while service.process do
      local success, message = xpcall(service.process, debug.traceback)
      if not success then
        print(message)
      end
    end
  end)()

  server:recv_start(function (err, data, addr)
    if err then return print(err) end
    if not data then return end
    coroutine.wrap(function ()
      local success, error = xpcall(function ()
        local message = cbor.decode(data)
        p('<-', message)
        assert(type(message) == 'table' and #message >= 1, 'Message must be list')
        local channel_id = table.remove(message, 1)
        addrs[channel_id] = { addr.ip, addr.port }

        if type(channel_id) == 'number' then
          local name = assert(table.remove(message, 1))
          assert(type(name) == 'string')
          local fn = assert(service[name])
          local success, result = xpcall(function ()
            return { fn(unpack(message)) }
          end, debug.traceback)
          if success then
            send(channel_id, true, unpack(result))
          else
            print(result)
            send(channel_id, false, result:match("[^\n]+"))
          end
        elseif type(channel_id) == 'string' then
          service.on_hash(channel_id, unpack(message))
        end
      end, debug.traceback)
      if not success then
        print(error)
      end
    end)()
  end)

  p('UDP server bound', server:getsockname())

end
