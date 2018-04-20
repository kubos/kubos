local make_service = require '../.'
local cbor = require 'cbor'

local calls = {}
local meta = {}
function meta:__index(key)
  return function (...)
    local message = cbor.decode(cbor.encode{self.id, key, ...})
    p(message)
    table.insert(calls, message)
  end
end

local cache = {}
local function recorder(id)
  local service = cache[id]
  if not service then
    service = make_service(setmetatable({
      id = id,
    }, meta))
    cache[id] = service
  end
  return service
end

recorder(100).spawn('sleep', { args = { '1000' } })
recorder(200).list()
recorder(400).spawn('env', { env = { 'test=1' } })
recorder(400).list()
recorder(500).spawn('pwd', { cwd = '/' })
recorder(500).list()
recorder(300).spawn('sort')
recorder(200).list()
recorder(300).stdin('Banana\n')
recorder(300).stdin('Cherry\n')
recorder(300).stdin('Apple\n')
recorder(100).kill()
recorder(300).stdin()

require('uv').run()
recorder(200).list()
require('uv').run()

local paths = {
  [100] = 'sleep',
  [300] = 'sort',
  [400] = 'env',
  [500] = 'pwd',
}
local pids = {}
local exits = {}
local stdouts = {}
for i = 1, #calls do
  local id = assert(calls[i][1])
  local event = assert(calls[i][2])
  if event == 'pid' then
    assert(not pids[id])
    pids[id] = calls[i][3]
  elseif event == 'stdout' then
    local stdout = stdouts[id]
    if not stdout then
      stdout = {}
      stdouts[id] = stdout
    end
    local val = calls[i][3]
    if val then
      table.insert(stdout, val)
    else
      stdouts[id] = table.concat(stdout)
    end
  elseif event == 'stderr' then
    assert(not calls[i][3])
  elseif event == 'exit' then
    assert(not exits[id])
    pids[id] = nil
    exits[id] = { unpack(calls[i], 3) }
  elseif event == 'list' then
    for k, v in pairs(assert(calls[i][3])) do
      local pid = assert(pids[k])
      assert(v.pid == pid)
      local path = assert(paths[k])
      assert(v.path == path)
    end
  else
    error('Invalid event ' .. event)
  end
end

p {
  exits = exits,
  stdouts = stdouts
}

assert(stdouts[100] == '')
assert(exits[100][1] == 0)
assert(exits[100][2] == 15)
assert(stdouts[300] == 'Apple\nBanana\nCherry\n')
assert(exits[300][1] == 0)
assert(stdouts[400] == 'test=1\n')
assert(exits[400][1] == 0)
assert(stdouts[500] == '/\n')
assert(exits[500][1] == 0)
