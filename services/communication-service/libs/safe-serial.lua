local uv = require 'uv'
local fs = require 'coro-fs'
-- local defer = require 'defer'
local constants = require('uv').constants
local O_RDWR = constants.O_RDWR
local O_NOCTTY = constants.O_NOCTTY
local O_SYNC = constants.O_SYNC

local set_termio = require 'termios-serial'

local bit = require 'bit'
local bor = bit.bor
local sub = string.sub

return function (dev, baud)
  local mode = bor(O_RDWR, O_NOCTTY, O_SYNC)
  local fd = assert(fs.open(dev, mode))
  set_termio(fd, baud)

  local queue = {}
  local locked = false

  local function lock()
    -- p("lock", locked, #queue)
    if not locked then
      locked = true
      return
    end
    queue[#queue + 1] = coroutine.running()
    coroutine.yield()
  end

  local function unlock()
    -- p("unlock", locked, #queue)
    if #queue == 0 then
      locked = false
      return
    end
    local thread = table.remove(queue, 1)
    coroutine.resume(thread)
  end

  -- local function protect(fn)
  --   return function (...)
  --     lock()
  --     local res = { pcall(fn, ...) }
  --     defer(unlock)
  --     if res[1] then
  --       return unpack(res, 2)
  --     else
  --       error(res[2])
  --     end
  --   end
  -- end

  local buffer = ''
  local function read(num)
    local timer = uv.new_timer()
    local thread = coroutine.running()
    timer:start(60000, 0, function ()
      coroutine.resume(thread, nil, 'Read Timeout')
    end)
    while #buffer < num do
      local chunk = assert(fs.read(fd))
      -- p('Serial read', chunk)
      if not chunk then break end
      buffer = buffer .. chunk
    end
    local data
    if #buffer > num then
      data = sub(buffer, 1, num)
      buffer = sub(buffer, num + 1)
    else
      data = buffer
      buffer = ''
    end
    timer:close()
    return data
  end

  local function write(data)
    -- p('Serial write', data)
    if not data then
      return assert(fs.close(fd))
    end
    local bytes = assert(fs.write(fd, data))
    assert(bytes == #data)
  end

  return read, write, protect
end
