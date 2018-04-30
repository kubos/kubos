-- This transport was originally made for the ground station for the twxvu
-- radio.  It connects to a TCP port and speaks KISS framed AX.25 messages.
local codec = require 'codec-kiss-ax25'
local connect = require('coro-net').connect
local getaddrinfo = require('uv').getaddrinfo
local match = string.match
local gsub = string.gsub


local ipv4_pattern = '(%d+)%.(%d+)%.(%d+)%.(%d+)'
local ipv6_pattern = gsub('(%h*):(%h*):(%h*):(%h*):(%h*):(%h*):(%h*):(%h*)',
  '%%h', '[a-fA-F0-9]')

local function guess_host_family(host)
  assert(type(host) == 'string', 'Host must be a string')

  -- Check for dotted decimal IPv4 strings
  local chunks = { match(host, ipv4_pattern) }
  if #chunks == 4 then
    for _,v in pairs(chunks) do
      local num = tonumber(v)
      if not num or num < 0 or num > 255 then return end
    end
    return 'inet'
  end

  -- check for ipv6 format.
  chunks = { match(host, ipv6_pattern) }
  if #chunks <= 8 and #chunks > 1 then
    for _,v in pairs(chunks) do
      p{v=v}
      if #v > 0 then
        local num = tonumber(v, 16)
        if not num or num < 0 or num > 65535 then return end
      end
    end
    return 'inet6'
  end
end

return function (host, port)
  assert(host, 'Missing host paremeter')
  assert(port, 'Missing port parameter')
  port = assert(tonumber(port), 'port is not a number')
  print 'Connecting to TCP socket speaking kiss framed ax.25'

  local family = guess_host_family(host)

  if not family then
    local info = unpack(getaddrinfo(host, nil, {socktype = 'stream'}))
    if info then
      host = info.addr
      family = info.family
    end
  end
  print 'TCP endpoint:'
  p {
    host = host,
    port = port,
    family = family,
  }
  return assert(connect {
    host = host,
    port = port,
    family = family,
    encode = codec.encode,
    decode = codec.decode,
  })
end
