local getenv = require('os').getenv
local uv = require 'uv'
local cbor = require 'cbor'

local file_protocol = require './.'

-- { hash }, -- syn
-- { hash, num_chunks }, -- syn
-- { hash, chunk_index, data }, -- send chunk no reply needed
-- { hash, true, num_chunks }, -- ack
-- { hash, false, 1, 4, 6, 7 }, -- nak
-- { channel_id, "export", hash, path, mode } -- mode is optional
-- { channel_id, "import", path }, --> returns file hash, num_chunks
-- { channel_id, true, value },
-- { channel_id, false, error_message},

local server = uv.new_udp()
local service_port = getenv 'PORT'
service_port = service_port and tonumber(service_port) or 7000
assert(server:bind('127.0.0.1', service_port))

-- Map from channel_id/hash to {ip,port}
local addrs = {}

-- Expire address table entries after a period of inactivity
local timer = uv.new_timer()
timer:start(1000, 1000, function ()
  local new_addrs = {}
  local expire = uv.now() - 10000
  for k, v in pairs(addrs) do
    if v[3] > expire then
      new_addrs[k] = v
    end
  end
  addrs = new_addrs
end)
timer:unref()

local function send(channel_id, ...)
  local message = {channel_id, ...}
  p('->', message)
  local addr = assert(addrs[channel_id], 'Unknown receiver address')
  local ip, port = unpack(addr)
  addrs[channel_id][3] = uv.now()
  local encoded = cbor.encode(message)
  server:send(encoded, ip, port)
end

local protocol = file_protocol(send, 'storage')

server:recv_start(function (err, data, addr)
  if err then return print(err) end
  if not data then return end
  coroutine.wrap(function ()
    local success, error = xpcall(function ()
      local message = cbor.decode(data)
      p('<-', message)
      assert(type(message) == 'table' and #message > 0)
      addrs[message[1]] = { addr.ip, addr.port, uv.now() }
      protocol.on_message(message)
    end, debug.traceback)
    if not success then
      print(error)
    end
  end)()
end)

require('uv').run()
