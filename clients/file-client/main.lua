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
local getenv = require('os').getenv
local fs = require 'coro-fs'
local splitPath = require('pathjoin').splitPath

local port = getenv 'PORT'
if port then port = tonumber(port) end
if not port then port = 7000 end

local file_protocol = require 'kubos-file-service'

local handle = uv.new_udp()
handle:bind('127.0.0.1', 0)
p(handle:getsockname())

local function send(...)
  local message = {...}
  p("->", message)
  handle:send(cbor.encode(message), "127.0.0.1", port)
end

local protocol = file_protocol(send, 'storage')

handle:recv_start(function (err, data, addr)
  if err then return print(err) end
  if not data then return end
  p(addr)
  assert(addr.port == port)
  coroutine.wrap(function ()
    local success, error = xpcall(function ()
      local message = cbor.decode(data)
      p('<-', message)
      assert(type(message) == 'table' and #message > 0)
      protocol.on_message(message)
    end, debug.traceback)
    if not success then
      print(error)
    end
  end)()
end)

local function upload(path, target_path)
  if not target_path then
    local parts = splitPath(path)
    target_path = parts[#parts]
  end
  p(path, target_path)
  local mode = assert(fs.stat(path)).mode
  local hash, num_chunks = protocol.local_import(path)
  protocol.send_sync(hash, num_chunks)
  protocol.call_export(hash, target_path, mode)
end

local usage = [[
Kubos File Client Utility Usage:

  kubos-file-client upload path/to/local/file.txt [/remote/path/file.txt]
  kubos-file-client download /remote/path/file.txt [local/path/file.txt]
]]

coroutine.wrap(function ()
  local success, message = xpcall(function ()
    local command = args[1]
    if command == 'upload' and #args >= 2 then
      upload(args[2], args[3])
    elseif command == 'download' and #args >= 2 then
      download(args[2], args[3])
    else
      print(usage)
    end
    handle:close()
  end, debug.traceback)
  if not success then
    print(message)
  end
end)()

uv.run()
