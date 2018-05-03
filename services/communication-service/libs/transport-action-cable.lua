local uv = require 'uv'
local ws = require 'coro-websocket'
local wrap = require('action-frame').wrap
local subscribe = require('action-frame').subscribe
local make_callback = require 'make-callback'

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

  local function connect()
    local timeout = 100
    local res, read, write
    while true do
      res, read, write = ws.connect(options)
      if res then break end
      print(read, "Trying again in " .. timeout .. 'ms')
      local timer = uv.new_timer()
      timer:start(timeout, 0, make_callback())
      coroutine.yield()
      timer:close()
      timeout = timeout * 2
    end
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
      connect()
    end)()
  end

  connect()

  return io
end
