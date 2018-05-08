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

-- This transport is for communicating over nsl duplex radio over serial.  Simply point it
-- to `/dev/ttyUSB*` or `/dev/ttyUSBO*` or whatever device you want with the
-- agreed upon baud rate and it will open the device file and configure it.

local uv = require 'uv'
local new_radio = require 'nsl-serial'

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

local function sleep(ms)
  local thread = coroutine.running()
  local timer = uv.new_timer()
  timer:start(ms, 0, function ()
    coroutine.resume(thread)
  end)
  coroutine.yield()
  timer:close()
end

-- config.device - The `/dev/tty*` device to connect to
-- config.baud - The baud rate. (38400 is the nsl default)
return function (config)
  local io = {}

  local device = assert(config.device, 'missing device argument for nsl transport')
  local baud = 38400
  baud = assert(tonumber(baud), 'baud is not a number')

  p("grabbing radio serial")
  local radio = new_radio(device, baud)
  local get_uploaded_file_count = radio.get_uploaded_file_count
  local get_uploaded_file = radio.get_uploaded_file
  local get_uploaded_message_count = radio.get_uploaded_message_count
  local get_uploaded_message = radio.get_uploaded_message
  local get_download_file_count = radio.get_download_file_count
  local put_download_file = radio.put_download_file
  local get_state_of_health_for_modem = radio.get_state_of_health_for_modem
  local get_geolocation_position_estimate = radio.get_geolocation_position_estimate

  local read, write
  local count = 1

  function read()
    while true do
      sleep(1000)
      local ufile = get_uploaded_file_count()
      p("Upload File Count", ufile)
      if ufile > 0 then
        local name, body = get_uploaded_file()
        p("Received file", name)
        return body
      end
      sleep(1000)
      local umessage = get_uploaded_message_count()
      p("Upload Message Count", umessage)
      if umessage > 0 then
        p("message", get_uploaded_message())
      else
        sleep(1000)
        local dfile = get_download_file_count()
        p("Download File Count", dfile)
        -- if dfile == 0 then
        --   local name = 'K' .. count
        --   assert(put_download_file(name, ''))
        -- end
        sleep(1000)
        local health = get_state_of_health_for_modem()
        p(health)
        local geo = get_geolocation_position_estimate()
        p(geo)
      end
    end
  end

  function write(data)
    local name = 'UDP' .. count
    count = count + 1
    p("Sending file", name)
    assert(put_download_file(name, data))
  end

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
