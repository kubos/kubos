local uv = require 'uv'
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
  expiresIn = 60 * 60 * 24 * 30 * 6, -- expires in 6 months
  secret = shared_secret
})

print("Token: ", token)

function on_gateway(id, read, write)
  p("Gateway", id, read, write)
  write {
    dest = 6000,
    source = 5000,
    data = '\0' .. CBOR.encode {
      uv.hrtime() % 0x10000, "spawn", "ls"
    }
  }
  for message in read do
    p(CBOR.decode(message.data, 2))
  end
end
