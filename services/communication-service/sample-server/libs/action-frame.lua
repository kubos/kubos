-- This simple library provides framing and deraming for binary data to pass
-- over action_cable in Major Tom.

local JSON = require 'json'
local Base64 = require 'base64'
local UDP = require 'codec-udp'

local action_frame = JSON.stringify {
  command = 'message',
  identifier = JSON.stringify {
    channel = 'GatewayChannel'
  },
  data = JSON.stringify {
    packet = '%', -- placeholder
    action = 'message' },
}

local function encode(packet)
  return action_frame:gsub('%%', Base64.encode(UDP.encode(packet)))
end

local function decode(json)
  return UDP.framed_decode(
    Base64.decode(json:match('\\"packet\\"%w*:%w*\\"([^"]*)\\"')),
    1)
end

return {
  encode = encode,
  decode = decode,
}
