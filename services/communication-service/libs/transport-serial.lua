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

return function (dev, baud)
  assert(dev, 'missing device argument to serial transport')
  assert(baud, 'missing baud argument to serial transport')
  baud = assert(tonumber(baud), 'baud is not a number')
  local mode = bor(O_RDWR, O_NOCTTY, O_SYNC)
  local fd = assert(fs.open(dev, mode))
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
  print 'Serial transport setup:'
  p {
    dev = dev,
    baud = baud,
    fd = fd
  }
  read = decoder(read, kiss.decode)
  write = encoder(write, kiss.encode)
  return read, write
end
