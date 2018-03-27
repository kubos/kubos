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
