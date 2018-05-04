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

local UDP = require 'codec-udp'

-- This simple library provides framing and deraming for binary data to pass
-- over action_cable in Major Tom.

local JSON = require 'json'
local Base64 = require 'base64'

local identifier = JSON.stringify { channel = 'GatewayChannel' }

local subscribe = JSON.stringify {
  command = 'subscribe',
  identifier = identifier
}

local action_frame = JSON.stringify {
  command = 'message',
  identifier = identifier,
  data = JSON.stringify {
    action = 'message',
    packet = '%' -- placeholder
  }
}

local function encode(frame)
  return action_frame:gsub('%%', Base64.encode(UDP.encode(frame)))
end

local function decode(json)
  return UDP.framed_decode(Base64.decode(json:match('\\"packet\\"%w*:%w*\\"([^"]*)\\"')), 1)
end

local function wrap(read, write)
  -- accepts { source = ..., dest = ..., data = ...}
  local function send(packet)
    if not packet then return write() end
    return write {
      opcode = 1,
      payload = encode(packet),
    }
  end

  -- returns { source = ..., dest = ..., data = ..., checksum = ... }
  local function receive()
    while true do
      local message = read()
      if not message then return end
      if message.opcode == 1 then
        return decode(message.payload)
      end
    end
  end

  return receive, send
end

return {
  subscribe = subscribe,
  encode = encode,
  decode = decode,
  wrap = wrap,
}
