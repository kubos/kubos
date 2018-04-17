local fs = require 'coro-fs'
local uv = require 'uv'
local join = require('pathjoin').pathJoin
local Blake2s = require 'blake2s'
local Bitfield = require 'bitfield'

local storage_path = 'storage'

local function ensure_dir(hash)
  local hash_path = join(storage_path, hash)
  assert(fs.mkdirp(hash_path))
  return hash_path
end

local function store_chunk(hash, index, data)
  assert(type(hash) == 'string')
  assert(type(index) == 'number')
  assert(type(data) == 'string')
  assert(fs.writeFile(join(ensure_dir(hash), string.format('%x', index)), data))
end

local function store_meta(hash, num_chunks)
  assert(type(hash) == 'string')
  assert(type(num_chunks) == 'number')
  assert(fs.writeFile(join(ensure_dir(hash), 'meta'), string.format('%x', num_chunks)))
end

local function load_chunk(hash, index)
  assert(type(hash) == 'string')
  assert(type(index) == 'number')
  local chunk_path = join(storage_path, hash, string.format('%x', index))
  return assert(fs.readFile(chunk_path))
end

local function load_meta(hash)
  assert(type(hash) == 'string')
  local meta_path = join(storage_path, hash, 'meta')
  return assert(tonumber(assert(fs.readFile(meta_path)), 16))
end



-- check what chunks are missing.  Also store_meta
local function sync(hash, num_chunks)

  if num_chunks then
    store_meta(hash, num_chunks)
  else
    num_chunks = load_meta(hash)
  end

  local bits = Bitfield.new(num_chunks)

  local hash_path = join(storage_path, hash)
  for entry in fs.scandir(hash_path) do
    local index = tonumber(entry.name, 16)
    if index then
      bits:set(index, true)
    end
  end
  local last = true
  local ranges = {}
  for i = 0, num_chunks - 1 do
    local this = bits:get(i)
    if this ~= last then
      ranges[#ranges + 1] = i
      last = this
    end
  end
  if #ranges % 2 == 1 then
    ranges[#ranges + 1] = num_chunks
  end
  return ranges
end

-- combine chunks and write to target path
local function export(hash, path, mode)
  assert(type(hash) == 'string')
  assert(type(path) == 'string')
  if mode then
    assert(type(mode) == 'number')
  else
    mode = 0x1a4 -- 0o644
  end
  local output = assert(fs.open(path, 'w', mode))
  local num_chunks = load_meta(hash)
  local h = Blake2s.new(16)
  for i = 0, num_chunks - 1 do
    local chunk = load_chunk(hash, i)
    h:update(chunk)
    assert(fs.write(output, chunk))
  end
  fs.close(output)
  local actual_hash = h:digest('hex')
  assert(actual_hash == hash, 'hash mismatch')
end

-- create temporary folder for chunks
-- stream copy file from mutable space to immutable space
-- move folder to hash of contents
local function import(path)
  local temp_path, input, output, hash, index
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
    hash = h:digest('hex')
    fs.close(input)
    input = nil

    -- Import chunks from temp file into storage
    index = 0
    local offset = 0
    while true do
      local chunk = assert(fs.read(output, 4096, offset))
      if #chunk == 0 then break end
      store_chunk(hash, index, chunk)
      index = index + 1
      offset = offset + #chunk
    end
    store_meta(hash, index)
  end, debug.traceback)
  if input then fs.close(input) end
  if output then fs.close(output) end
  if temp_path then fs.unlink(temp_path) end
  if not success then
    error(message)
  end
  return hash, index
end

return {
  store_chunk = store_chunk,
  load_chunk = load_chunk,
  sync = sync,
  import = import,
  export = export
}
