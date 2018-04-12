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

-- Listens for udp packets over a custom transport and forwards to UDP services
-- on the local device.

local uv = require 'uv'
local getenv = require('os').getenv
local udp = require 'codec-udp'
local encoder = require('coro-wrapper').encoder
local decoder = require('coro-wrapper').decoder

local usage = [[
Please specify transport and options:

# Communicate via a serial device
kubos-communication-service serial /dev/ttyUSB0 115200

# Take over the current PTY assuming it's serial
# You need to redirect stderr somewhere so it doesn't dirty the serial data.
kubos-communication-service debug-serial 115200 2> debug-log
]]

local transport_name = args[1]
if not transport_name then
  print(usage)
  return -1
end

local ffi = require 'ffi'
ffi.cdef[[
  void exit(int status);
]]

local function wrap(fn)
  return function (...)
    local args = {...}
    return coroutine.wrap(function ()
      local res, err = xpcall(function ()
        return fn(unpack(args))
      end, debug.traceback)
      if not res then
        print(err)
        return ffi.C.exit(-1)
      end
    end)()
  end
end

local read, write

local handles = {}

local function make_receiver(dest)
  return wrap(function (err, data, addr)
    if err then return print(err) end
    if not data then return end
    local source = addr.port
    p('udp-res -> ' .. transport_name, {source=source, dest=dest, len=#data})
    write {
      source = source,
      dest = dest,
      data = data
    }
  end)
end

local function make_sender(dest)
  return wrap(function (err, data, addr)
    assert(not err, err)
    if not data then return end
    local source = addr.port
    p('udp-req -> ' .. transport_name, {
      source = source,
      dest = dest,
      len = #data
    })
    write {
      source = source,
      dest = dest,
      data = data,
    }
  end)
end

wrap(function ()
  -- Setup the custom transport
  local transport = require('transport-' .. transport_name)
  read, write = transport(unpack(args, 2))
  read = decoder(read, udp.decode)
  write = encoder(write, udp.encode)

  -- Expose remote ports locally if requested via EXPOSE_PORTS env variable.
  local host = getenv("HOST") or "127.0.0.1"
  local expose = getenv 'EXPOSE_PORTS'
  if expose then
    for dest in expose:gmatch("%d+") do
      dest = tonumber(dest)
      local server = uv.new_udp()
      assert(server:bind(host, dest))
      assert(server:recv_start(make_sender(dest)))
      print 'Communications service forwarding UDP:'
      p("repeater", server:getsockname())
    end
  end

  -- Listen for messages over the custom transport
  for message in read do
    local source = message.source
    local dest = message.dest
    local data = message.data
    local checksum = message.checksum
    local handle = handles[source]
    if not handle then
      handle = uv.new_udp()
      assert(handle:bind('127.0.0.1', 0))
      assert(handle:recv_start(make_receiver(source)))
      handles[source] = handle
      p('new handle', handle:getsockname())
    end
    -- If dest address is zero, this is signal to cleanup mapping
    if dest == 0 then
      handles[source] = nil
      handle:close()
    else
      p(transport_name .. ' -> udp', {
        source = source,
        dest = dest,
        len = #data,
        checksum = checksum
      })
      handle:send(data, '127.0.0.1', dest)
    end
  end

end)()

uv.run()
