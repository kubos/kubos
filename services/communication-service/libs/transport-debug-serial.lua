-- This transport is for comminicating over a debug serial port.  It's run
-- by connecting to a debug port from the outside over serial using screen,
-- logging in and starting a process to take over stdio.  This will redirect all
-- `print(...)` and `p(...)` calls to stderr so they don't corrupt the transport
-- data.  It will also set the baud rate of stdout and configure the serial
-- termios.

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

return function (baud)
  assert(baud, 'Missing baud parameter')
  baud = assert(tonumber(baud), 'baud is not a number')
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

  local read = decoder(wrapRead(stdin), kiss.decode)
  local write = encoder(wrapWrite(stdout), kiss.encode)
  return read, write
end
