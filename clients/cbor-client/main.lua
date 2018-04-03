local uv = require 'uv'
local dump = require('pretty-print').dump
local Editor = require('readline').Editor
local cbor = require 'cbor'

-- Default lua strings to utf8 strings in cbor encoding.
cbor.type_encoders.string = cbor.type_encoders.utf8string

local usage = [[
Kubos CBOR Client
This is a tiny tool that provides a line interface to CBOR-UDP services. It can
talk to local services or remote services through the communications bridge.
Type messages in lua syntax and they will be parsed as lua, encoded as cbor and
sent over the wire.  Responses will be parsed as cbor and printed as lua.

Usage: kubos-cbor-client $service_port
]]

local port = args[1]
port = port and tonumber(port)
if not port then
  print(usage)
  return -1
end
local editor = Editor.new {}
local prompt = 'udp:' .. port .. '> '

local udp = uv.new_udp()

assert(udp:bind('127.0.0.1', 0))

local function on_line(err, line, reason)
  assert(not err, err)
  if reason == 'EOF in readLine' then
    print 'Exiting...'
    udp:recv_stop()
    udp:close()
    return
  end
  local fn, error = loadstring('return ' .. line)
  if not fn then
    editor:insertAbove(error)
  else
    setfenv(fn, {})
    local success, value = pcall(fn)
    if success then
      editor:insertAbove('Client: ' .. dump(value))
      udp:send(cbor.encode(value), '127.0.0.1', port)
    else
      editor:insertAbove(value)
    end
  end
  editor:readLine(prompt, on_line)
end


udp:recv_start(function (err, data)
  assert(not err, err)
  if not data then return end
  local value = cbor.decode(data)
  editor:insertAbove('Server: ' .. dump(value))
end)

editor:readLine(prompt, on_line)

uv.run()
