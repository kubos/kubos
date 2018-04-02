local uv = require 'uv'
local dump = require('pretty-print').dump
local Editor = require('readline').Editor

local usage = [[
Kubos UDP Client
This is a tiny tool that provides a line interface to UDP services. It can talk
to local services or remote services through the communications bridge.

Usage: kubos-udp-client $service_port
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
  udp:send(line, '127.0.0.1', port)
  editor:readLine(prompt, on_line)
end


udp:recv_start(function (err, data, addr)
  assert(not err, err)
  if not data then return end
  editor:insertAbove(dump(addr))
  editor:insertAbove(data)
end)

editor:readLine(prompt, on_line)

uv.run()
