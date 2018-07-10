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

-- This codec implements the SLIP or KISS protocol. It's called SLIP when
-- used on a serial link and KISS when used on radio but it's the same protocol.
-- The purpose is to provide basic message framing as well as stream syncing.

local concat = table.concat
local gsub = string.gsub
local sub = string.sub
local find = string.find

local function kiss_escape(c)
  if c == '\xc0' then return '\xdb\xdc' end
  if c == '\xdb' then return '\xdb\xdd' end
  return c
end

local function decode(chunk, index)
  local frame
  repeat
    -- Look for first full kiss frame
    local a, b = find(chunk, '\xc0\x00', index)
    if not a then return end
    local c = find(chunk, '\xc0', b)
    if not c then return end

    -- Extract the frame payload
    frame = sub(chunk, b + 1, c - 1)
    index = c

    -- Unescape KISS control characters
    local valid = true
    frame = gsub(frame, '\xdb(.)', function (m)
      if m == '\xdc' then return '\xc0' end
      if m == '\xdd' then return '\xdb' end
      -- If an invalid escape is found, abort this frame
      valid = false
      return m
    end)

  until valid
  return frame, index
end

local function encode(frame)
  return concat {
    '\xc0\x00',
    gsub(frame, '.', kiss_escape),
    '\xc0'
  }
end

return {
  encode = encode,
  decode = decode,
}
