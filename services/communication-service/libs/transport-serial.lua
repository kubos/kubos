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

-- This transport is for communicating over a serial device.  Simply point it
-- to `/dev/ttyUSB*` or `/dev/ttyO*` or whatever device you want with the
-- agreed upon baud rate and it will open the device file and configure it.

local UDP = require 'codec-udp'
local fs = require 'coro-fs'
local constants = require('uv').constants
local O_RDWR = constants.O_RDWR
local O_NOCTTY = constants.O_NOCTTY
local O_SYNC = constants.O_SYNC
local bor = require('bit').bor

local set_termio = require 'termios-serial'
local kiss = require 'codec-slip'
local encoder = require('coro-wrapper').encoder
local decoder = require('coro-wrapper').decoder

-- config.device - The `/dev/tty*` device tto connect to.
-- config.baud - The baud rate. (115200, 9600, etc...)
return function (config)
  local io = {}

  local device = assert(config.device, 'missing device argument to serial transport')
  local baud = assert(config.baud, 'missing baud argument to serial transport')
  local mode = bor(O_RDWR, O_NOCTTY, O_SYNC)
  local fd = assert(fs.open(device, mode))
  set_termio(fd, baud)

  local read, write

  function read()
    local data = assert(fs.read(fd))
    -- p("Serial read", data)
    return data
  end

  function write(data)
    -- p("Serial write", data)
    if not data then
      return assert(fs.close(fd))
    end
    local bytes = assert(fs.write(fd, data))
    assert(bytes == #data)
  end
  p {
    device = device,
    baud = baud,
    fd = fd
  }

  io.receive = encoder(encoder(write, kiss.encode), UDP.encode)
  read = decoder(decoder(read, kiss.decode), UDP.framed_decode)

  coroutine.wrap(function ()
    for packet in read do
      io.send(packet)
    end
    io.send()
  end)()

  return io
end
