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
local getenv = require('os').getenv
local splitPath = require('pathjoin').splitPath

local port = getenv 'PORT'
if port then port = tonumber(port) end
if not port then port = 7000 end

local cbor_message_protocol = require 'cbor-message-protocol'
local file_protocol = require 'file-protocol'

local handle = uv.new_udp()
handle:bind('127.0.0.1', 0)
p(handle:getsockname())

local protocol

local function on_message(message, addr)
  assert(addr.port == port)
  coroutine.wrap(function ()
    local success, error = xpcall(function ()
      assert(type(message) == 'table' and #message > 0)
      protocol.on_message(message)
    end, debug.traceback)
    if not success then
      print(error)
    end
  end)()
end

local send_message = cbor_message_protocol(handle, on_message, true)

local function send(...)
  send_message({...}, "127.0.0.1", port)
end

protocol = file_protocol(send, 'storage')

local function upload(source_path, target_path)
  if not target_path then
    local parts = splitPath(source_path)
    target_path = parts[#parts]
  end
  print(string.format("Uploading local:%s to remote:%s", source_path, target_path))
  local hash, num_chunks, mode = protocol.local_import(source_path)
  protocol.send_sync(hash, num_chunks)
  protocol.call_export(hash, target_path, mode)
  print 'Upload Complete'
end

local function download(source_path, target_path)
  if not target_path then
    local parts = splitPath(source_path)
    target_path = parts[#parts]
  end
  print(string.format("Downloading remote:%s to local:%s", source_path, target_path))
  local hash, num_chunks, mode = protocol.call_import(source_path)
  protocol.sync_and_send(hash, num_chunks)
  protocol.local_export(hash, target_path, mode)
  print 'Download Complete'
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
