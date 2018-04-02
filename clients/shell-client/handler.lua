
local udp = require 'udp-codec'
local encoder = require('coro-wrapper').encoder
local decoder = require('coro-wrapper').decoder
local serial_stream = require 'coro-serial'
local getenv = require('os').getenv

return function (device, baud)
  -- Map of tcp client ports to client handles
  local clients = {}

  local write
  local read
  coroutine.wrap(function ()
    local stream = serial_stream(device, baud)
    read = stream.read
    write = stream.write
    if getenv 'HEX' then
      local hex = require 'hex-escape'
      print("Escaping serial data with hex")
      read = decoder(read, hex.decode)
      write = encoder(write, hex.encode)
    end
    read = decoder(read, udp.decode)
    write = encoder(write, udp.encode)
    for out in read do
      if not out.checksum then
        print("Warning invalid checksum", out.source, out.dest)
      end
      p("serial -> websocket", {source=out.source, dest=out.dest, len=#(out.data),checksum=out.checksum})
      local client = clients[out.dest]
      if client then
        client {
          opcode = 2,
          payload = out.data
        }
      else
        print("Warning, no known client for " .. out.dest)
      end
    end
  end)()

  return function (req, ws_read, ws_write)
    local dest = tonumber(req.params.port)
    local source = req.socket:getpeername().port
    clients[source] = ws_write
    p("New client", {dest=dest,source=source})
    for message in ws_read do
      if message.opcode == 2 then
        local data = message.payload
        p("websocket -> serial", {source=source, dest=dest, len=#data})
        write {
          source = source,
          dest = dest,
          data = data
        }
      end
    end
    p("Client left", {dest=dest,source=source})
    ws_write()
    write {
      source = source,
      dest = 0,
      data = ""
    }
    clients[source] = nil
  end
end
