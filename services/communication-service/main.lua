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
local fs = require 'coro-fs'
local toml = require 'toml'
local dump = require('pretty-print').dump

local usage = [=[
Please specify a config file:

    kubos-communication-service config.toml

Here is a sample config file that bridges a remote device connected via serial
with local UDP clients.

    [[communication-service]]
    name = "Beagle Bone Serial Host Side"
    type = "serial"
    device = "/dev/ttyUSB0"
    baud = 115200

    [[communication-service]]
    name = "Local UDP Clients"
    type = "udp"
    exports = [ 6000, 7000 ]
]=]


-- # Take over the current PTY assuming it's serial
-- # You need to redirect stderr somewhere so it doesn't dirty the serial data.
-- kubos-communication-service debug-serial 115200 2> debug-log

local config_file_path = args[1]
if not config_file_path then
  print(usage)
  return -1
end

local function load(transport)
  print(string.format("Loading transport: (%s) %s", transport.transport, transport.name or ''))
  local fn = require('transport-' .. transport.transport)
  return fn(transport)
end

local function link(io1, io2, transport1, transport2)
  io1.send = function (frame)
    print(string.format("%s(%s) -> %s(%s) %s bytes",
      transport1.transport,
      dump(frame.source),
      transport2.transport,
      dump(frame.dest),
      dump(#frame.data)
    ))
    return io2.receive(frame)
  end
end

coroutine.wrap(function ()
  local config = toml.parse(assert(fs.readFile(config_file_path)))
  local transports = config['communication-service']
  assert(transports, 'Missing `communication-service` in config file')
  assert(#transports == 2, '`communication-service` config needs to contain two transports')
  local io1 = load(transports[1])
  local io2 = load(transports[2])
  link(io1, io2, transports[1], transports[2])
  link(io2, io1, transports[2], transports[1])
  p(1, io)
  p(2, io)
end)()

uv.run()
