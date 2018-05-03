local jwt = require 'jwt'
local Gateway = require 'gateway'
local fs = require 'fs'
local CBOR = require 'cbor'

local shared_secret = 'kubos-rocks'

require 'weblit-websocket'

local on_gateway
local app = require('weblit-app')

app.bind {
  tls = {
    cert = fs.readFileSync('./cert.pem'),
    key = fs.readFileSync('./key.pem'),
  }
}

app.use(require 'weblit-logger')
app.use(require 'weblit-auto-headers')

-- Communication service will register with this websocket server using a JWT to
-- associate it with a given `gateway_id` inside Major Tom.
-- The UDP frames from the transport will tunnel over Action Cable frames.

local function authenticate_token(req, res, go)
  local token = jwt.verify(req.query.agent_token, { secret = shared_secret })
  p(token)
  if not token then
    res.code = 403
    res.body = "Bad token\n"
    return
  end
  req.gateway_id = assert(token.gateway_id)
  return go()
end

app.route({
  path = "/register"
}, authenticate_token)

app.websocket({
  path = "/register"
}, function (req, read, write)
  p(req)
  on_gateway(req.gateway_id, Gateway.new(req.gateway_id, read, write))
end)

app.start()

local token = jwt.sign({
  gateway_id = 42,
}, {
  expiresIn = 60 * 60, -- expires in an hour
  secret = shared_secret
})

print("Token: ", token)

function on_gateway(id, read, write)
  p("Gateway", id, read, write)
  write {
    dest = 6000,
    source = 5000,
    data = '\0' .. CBOR.encode {
      100, "spawn", "ls"
    }
  }
  for message in read do
    p(message)
  end
end
--
-- coroutine.wrap(function ()
--   --
--   -- local res, read, write = ws.connect {
--   --   host = "localhost",
--   --   port = 8080,
--   --   pathname = "/register?agent_token=" .. token
--   -- }
--   --
--   -- p(res)
--   -- read, write = Gateway.wrap(read, write)
--   -- p(read, write)
--   -- local receive, send = unpack(Gateway.get(42))
--   -- p(receive, send)
--   --
--   -- send {
--   --   source = 6000,
--   --   dest = 5000,
--   --   data = 'Hello'
--   -- }
--   --
--   -- p(read())
--   --
--   -- write {
--   --   source = 5000,
--   --   dest = 6000,
--   --   data = 'World'
--   -- }
--   --
--   -- p(receive())
--   --
--   -- require('uv').walk(function (handle) handle:close() end)
--
--   -- wrapIo = wrapIo,
--   -- connect = connect,
--
-- end)()
--
-- --
-- -- FROM PORT  |  To PORT
-- -- LENGTH     | checksum
-- -- PAYLOAD...
-- --
-- -- Browser wants to connect to shell service
-- -- GET ws://mt/sat/shell
-- -- web server assigns ID 42
-- -- web server looks up service port (6000)
-- -- for every ws frame, web server encodes body as udp (from 42 to 600)
-- --
-- -- {
-- --
-- -- }
-- --
-- -- gateway.on_message(packet)
-- -- gateway.send_message(packet)
-- --
