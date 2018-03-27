local bit = require 'bit'
local band = bit.band
local bor = bit.bor
local bnot = bit.bnot
local rshift = bit.rshift
local lshift = bit.lshift
local char = string.char
local byte = string.byte
local sub = string.sub
local concat = table.concat

local function u16(chunk, index)
  return bor(
    lshift(byte(chunk, index), 8),
    byte(chunk, index + 1)
  )
end

-- Calculate an internet checksum for the udp packet
local function check(source, dest, len, data)
  local sum = source + dest + len
  for i = 1, len - 8, 2 do
    sum = sum + u16(data, i)
  end
  while sum >= 0x10000 do
    sum = rshift(sum, 16) + band(sum, 0xffff)
  end
  return band(bnot(sum), 0xffff)
end

local function decode(chunk, index)
  p("udp-decode", chunk, index)

  -- Wait till we have at least the 8 header bytes
  local offset = index - 1
  local length = #chunk - offset
  if length < 8 then return end

  -- And wait till we have the entire datagram
  local len = u16(chunk, index + 4)
  if length < len then return end

  -- Read the rest of the datagram and parse
  local source = u16(chunk, index)
  local dest = u16(chunk, index + 2)
  local checksum = u16(chunk, index + 6)
  local data = sub(chunk, index + 8, index + len - 1)
  local sum = check(source, dest, len, data)
  return {
    source = source,
    dest = dest,
    data = data,
    checksum = sum == checksum
  }, index + len
end

local function encode(item)
  p("udp-encode", item)
  if not item then return end
  local source = assert(item.source, 'missing source')
  local dest = assert(item.dest, 'missing dest')
  local data = assert(item.data, 'missing data')
  local len = #data + 8
  local checksum = check(source, dest, len, data)
  return concat {
    char(rshift(source, 8)), -- UDP source
    char(band(source, 0xff)),
    char(rshift(dest, 8)), -- UDP dest
    char(band(dest, 0xff)),
    char(rshift(len, 8)), -- UDP length
    char(band(len, 0xff)),
    char(rshift(checksum, 8)), -- UDP checksum
    char(band(checksum, 0xff)),
    data -- UDP payload
  }
end

return {
  encode = encode,
  decode = decode
}
