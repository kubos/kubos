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
local ffi = require 'ffi'
local stdout = require('pretty-print').stdout
local stderr = require('pretty-print').stderr
local stdin = require('pretty-print').stdin
local cbor = require 'cbor'
local getenv = require('os').getenv
local readLine = require('readline').readLine

-- default lua strings to utf8 strings in cbor encoding
cbor.type_encoders.string = cbor.type_encoders.utf8string

local port = getenv 'PORT'
if port then port = tonumber(port) end
if not port then port = 6000 end

local id = uv.hrtime() % 0x10000

local handle = uv.new_udp()
handle:bind('127.0.0.1', 0)
-- p(handle:getsockname())

local function send(message)
  local data = cbor.encode { id, unpack(message) }
  handle:send(data, '127.0.0.1', port)
end

ffi.cdef[[
  void exit(int status);
]]

local handlers = {}

local function on_raw(err, data)
  assert(not err, err)
  send { 'stdin', data }
end

function handlers.pid(pid)
  p('Remote sh process:', {pid=pid})
  send { 'stdin', '\f' }
  -- stdout:write '\x1b[2J\x1b[;H'
  stdin:set_mode(1)
  stdin:read_start(on_raw)
end

function handlers.stdout(data)
  stdout:write(data)
end

function handlers.stderr(data)
  stderr:write(data)
end

function handlers.exit(code, signal)
  stdin:set_mode(0)
  print()
  p('Remote sh process exited:', {code=code,signal=signal})
  send { 'list' }
end

function handlers.error(error)
  stdin:set_mode(0)
  print()
  print('Remote error: ' .. error)
  ffi.C.exit(-1)
end

function handlers.list(processes)
  print '\x1b[2J\x1b[;HChoose an option:'
  print 'Press enter to start a new sh shell.'
  print 'Press Control-D to exit'
  print 'Or enter session ID to take over an existing session.'
  for k, v in pairs(processes) do
    p(k, v)
  end
  local function onReadLine(err, out, reason)
    assert(not err, err)
    if reason == 'EOF in readLine' then
      print()
      return ffi.C.exit(0)
    end
    if out == '' then
      print 'Starting new remote sh shell...'
      send {
        'spawn',
        'sh',
        {
          args = { '-l' },
          pty = true,
          detached = true
        }
      }
      return
    end
    local option = tonumber(out)
    local proc = processes[option]
    if not proc then
      print 'Invalid option'
      return send { 'list' }
    end

    id = option
    handlers.pid(proc.pid)
  end
  readLine("> ", onReadLine)
end

handle:recv_start(function (err, data)
  assert(not err, err)
  if not data then return end
  local message = cbor.decode(data)
  local rid = message[1]
  if rid ~= id then return end
  local command = message[2]
  local fn = handlers[command]
  if type(fn) ~= 'function' then
    p(command)
    print('Unhandled command: ' .. command)
    return
  end
  fn(unpack(message, 3))
end)

send { 'list' }

-- local cols, rows = stdin:get_winsize()
-- send { 'resize', cols, rows }

uv.run()
