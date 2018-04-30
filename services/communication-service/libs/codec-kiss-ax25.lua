local kiss = require 'codec-slip' -- SLIP and KISS are the same protocol
local bit = require 'bit'
local rshift = bit.rshift
local lshift = bit.lshift
local band = bit.band
local bor = bit.bor
local char = string.char
local byte = string.byte
local sub = string.sub
local concat = table.concat

local function encode(message)
  local dest = assert(message.dest, "Missing `dest`")
  assert(type(dest) == 'table', '`dest` must be table')
  local d_callsign = assert(dest.callsign, 'Missing `dest.callsign`')
  assert(type(d_callsign) == 'string', '`dest.callsign` must be string')
  local d_ssid = assert(dest.ssid, 'Missing `dest.ssid`')
  assert(type(d_ssid) == 'number', '`dest.ssid` must be number')
  local d_bit = dest.bit
  assert(type(d_bit) == "boolean", "`dest.bit` must be boolean")
  local source = assert(message.source, "Missing `source`")
  assert(type(source) == 'table', '`source` must be table')
  local s_callsign = assert(source.callsign, 'Missing `source.callsign`')
  assert(type(s_callsign) == 'string', '`source.callsign` must be string')
  local s_ssid = assert(source.ssid, 'Missing `source.ssid`')
  assert(type(s_ssid) == 'number', '`source.ssid` must be number')
  local s_bit = source.bit
  assert(type(s_bit) == "boolean", "`source.bit` must be boolean")
  local data = assert(message.data, "Missing `data`")
  assert(type(data) == 'string', "`data` must be string")
  local frame = concat {
    char(lshift(byte(d_callsign, 1) or 0x20, 1)),
    char(lshift(byte(d_callsign, 2) or 0x20, 1)),
    char(lshift(byte(d_callsign, 3) or 0x20, 1)),
    char(lshift(byte(d_callsign, 4) or 0x20, 1)),
    char(lshift(byte(d_callsign, 5) or 0x20, 1)),
    char(lshift(byte(d_callsign, 6) or 0x20, 1)),
    char(bor(lshift(d_ssid, 1), d_bit and 0xe0 or 0x60)),
    char(lshift(byte(s_callsign, 1) or 0x20, 1)),
    char(lshift(byte(s_callsign, 2) or 0x20, 1)),
    char(lshift(byte(s_callsign, 3) or 0x20, 1)),
    char(lshift(byte(s_callsign, 4) or 0x20, 1)),
    char(lshift(byte(s_callsign, 5) or 0x20, 1)),
    char(lshift(byte(s_callsign, 6) or 0x20, 1)),
    char(bor(lshift(s_ssid, 1), s_bit and 0xe1 or 0x61)),
    "\x03\xf0",
    data
  }
  return kiss.encode(frame)
end

local function parse_callsign(chunk, index)
  local callsign = concat {
    char(rshift(byte(chunk, index), 1)),
    char(rshift(byte(chunk, index + 1), 1)),
    char(rshift(byte(chunk, index + 2), 1)),
    char(rshift(byte(chunk, index + 3), 1)),
    char(rshift(byte(chunk, index + 4), 1)),
    char(rshift(byte(chunk, index + 5), 1)),
  }
  local ssid = rshift(band(byte(chunk, index + 6), 0x1e), 1)
  return {
    callsign = callsign,
    ssid = ssid,
    bit = rshift(byte(chunk, index + 6), 7) == 1
  }
end

local function decode(chunk, index)
  local frame
  frame, index = kiss.decode(chunk, index)
  if not frame then return end
  return {
    dest = parse_callsign(frame, 1),
    source = parse_callsign(frame, 8),
    data = sub(frame, 17)
  }, index
end

return {
  encode = encode,
  decode = decode
}
