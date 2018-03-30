local uv = require 'uv'
local getenv = require('os').getenv
local codec = require 'http-codec'
local request = require('coro-http').request

local usage = [[
Kubos UDP-HTTP Relay:
  Exposes locahost http servers as UDP services.

Usage:
  PORT=4000 kubos-udp-http-relay
]]

local PORT = getenv 'PORT'
PORT = PORT and tonumber(PORT)
if not PORT then
  print(usage)
  return -1
end

p { PORT = PORT }

local server = uv.new_udp()
assert(server:bind('127.0.0.1', PORT))

local function parse_request(req)
  local decode = codec.decoder()
  local head, index = decode(req, 1)
  if not head then
    error 'Invalid HTTP message'
  end
  local host
  for _, h in ipairs(head) do
    local key, value = unpack(h)
    if key:lower() == 'host' then
      host = value
      break
    end
  end
  if not host then
    error 'Missing HTTP Host header'
  end
  local port = host:match(":(%d+)$")
  if not port then
    error 'Missing port in HTTP Host header'
  end
  port = tonumber(port)
  local chunks = {}
  while true do
    local chunk
    chunk, index = decode(req, index)
    if not chunk or #chunk == 0 then break end
    table.insert(chunks, chunk)
  end
  local body = table.concat(chunks)
  return port, head, body
end

-- Relay data in req to http server (extract port from headers)
--
local function relay(req, addr)
  local port, head, body = parse_request(req)
  head[#head + 1] = {"Connection", "close"}
  head, body = request(
    head.method, "http://127.0.0.1:" .. port .. head.path,
    head, body, 2000)
  local encode = codec.encoder()
  server:send(table.concat {
    encode(head),
    encode(body),
    encode('')
  }, addr.ip, addr.port)
end

server:recv_start(function (err, data, addr)
  assert(not err, err)
  if not data then return end
  coroutine.wrap(function ()
    local success, error = pcall(relay, data, addr)
    if not success then
      print(error)
      error = error .. '\r\n'
      server:send(table.concat {
        'HTTP/1.1 500 Internal Server Error\r\n',
        -- 'Date: Sun, 18 Oct 2012 10:36:20 GMT',
        'Server: Kubos-Udp-Http-Relay\r\n',
        'Content-Length: ' .. #error .. '\r\n',
        'Connection: Closed\r\n',
        'Content-Type: text/plain\r\n\r\n',
        error
      }, addr.ip, addr.port)
    end
  end)()
end)

uv.run()
