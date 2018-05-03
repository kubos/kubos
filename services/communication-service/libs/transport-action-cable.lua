local ws = require 'coro-websocket'
local wrap = require('action-frame').wrap
local subscribe = require('action-frame').subscribe


return function (config)
  -- The framework will populate this with io.send(frame)
  local io = {}

  local url = assert(config.url, 'Missing URL')
  local token = assert(config.token, 'Missing token')
  local options = ws.parseUrl(url)

  p("action-cable", options)
  options.pathname = options.pathname .. '?agent_token=' .. token

  -- TODO use server cert instead of insecure flag
  options.tls = { insecure = true }
  options.headers = {{"User-Agent", "kubos-communication-service"}}

  local res, read, write = ws.connect(options)
  assert(res.code == 101)

  write {
    opcode = 1,
    payload = subscribe
  }

  read, write = wrap(read, write)

  io.receive = write

  coroutine.wrap(function ()
    for frame in read do
      io.send(frame)
    end
    io.send()
  end)()

  return io
end
