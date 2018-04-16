local fs = require 'coro-fs'
local uv = require 'uv'
local join = require('pathjoin').pathJoin
local Blake2s = require 'blake2s'

local storage_path = 'storage'

local function store(hash, index, data)
  local hash_path = join(storage_path, hash)
  assert(fs.mkdirp(hash_path))
  assert(fs.writeFile(join(hash_path, string.format('%x', index)), data))
end

-- create temporary folder for chunks
-- stream copy file from mutable space to immutable space
-- move folder to hash of contents
local function import(path)
  local temp_path, input, output
  local success, message = xpcall(function ()

    -- Copy the input file to storage area and calculate hash
    assert(fs.mkdirp(storage_path))
    temp_path = join(storage_path, '.' .. uv.hrtime())
    input = assert(fs.open(path))
    output = assert(fs.open(temp_path, 'ax+'))
    local h = Blake2s.new(16)
    repeat
      local chunk = assert(fs.read(input))
      h:update(chunk)
      assert(fs.write(output, chunk))
    until #chunk == 0
    local hash = h:digest('hex')
    fs.close(input)
    input = nil

    -- Import chunks from temp file into storage
    local index = 0
    local offset = 0
    while true do
      local chunk = assert(fs.read(output, 4096, offset))
      if #chunk == 0 then break end
      store(hash, index, chunk)
      index = index + 1
      offset = offset + #chunk
    end

  end, debug.traceback)
  if input then fs.close(input) end
  if output then fs.close(output) end
  if temp_path then fs.unlink(temp_path) end
  if not success then
    error(message)
  end
end

coroutine.wrap(function ()
  local success, message = xpcall(function ()

    p(import("EyeStar-D2_Duplex_ICD_v7.8.pdf"))

  end, debug.traceback)
  if not success then
    print(message)
  end
end)()
