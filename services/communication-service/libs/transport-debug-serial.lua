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

-- This transport is for comminicating over a debug serial port.  It's run
-- by connecting to a debug port from the outside over serial using screen,
-- logging in and starting a process to take over stdio.  This will redirect all
-- `print(...)` and `p(...)` calls to stderr so they don't corrupt the transport
-- data.  It will also set the baud rate of stdout and configure the serial
-- termios.

-- To use this transport, screen into the device using the debug serial.
--
--     screen /dev/ttyUSB0 115200
--
-- In the shell session, run this transport and redirect stderr to a log file.
--
--     kubos-communication-service config.toml 2> comm-logs
--
-- And then leave that running and exit the screen session by pressing
-- Control-A and Control-K
--
-- Then start the host side of the communication service using the normal serial
-- transport.

local UDP = require 'codec-udp'
local uv = require 'uv'
local stdin = require('pretty-print').stdin
local stdout = require('pretty-print').stdout
local stderr = require('pretty-print').stderr
local dump = require('pretty-print').dump
local wrapRead = require('coro-channel').wrapRead
local wrapWrite = require('coro-channel').wrapWrite
local encoder = require('coro-wrapper').encoder
local decoder = require('coro-wrapper').decoder
local kiss = require 'codec-slip'
local set_termio = require 'termios-serial'

-- config.baud - The baud rate. (115200, 9600, etc...)
return function (config)
  local io = {}
  local baud = assert(config.baud, 'Missing baud parameter')
  print "Setting raw mode, you can no longer kill this with Control + C"
  set_termio(0, baud)

  print "Redirecting print and p to stderr.  Please redirect to file.\r"

  function _G.print(...)
    local n = select('#', ...)
    local arguments = {...}
    for i = 1, n do
      arguments[i] = tostring(arguments[i])
    end
    uv.write(stderr, table.concat(arguments, "\t") .. "\n")
  end

  function _G.p(...)
    local n = select('#', ...)
    local arguments = {...}
    for i = 1, n do
      arguments[i] = dump(arguments[i])
    end
    uv.write(stderr, table.concat(arguments, "\t") .. "\n")
  end

  io.receive = encoder(encoder(wrapWrite(stdout), kiss.encode), UDP.encode)
  local read = decoder(decoder(wrapRead(stdin), kiss.decode), UDP.framed_decode)

  coroutine.wrap(function ()
    for packet in read do
      io.send(packet)
    end
    io.send()
  end)()

  return io
end
