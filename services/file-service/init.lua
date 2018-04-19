local fs = require 'coro-fs'
local os = require 'os'
local uv = require 'uv'
local join = require('pathjoin').pathJoin
local Blake2s = require 'blake2s'
local Bitfield = require 'bitfield'
local bin = require('cbor').bin
local buf = require('cbor').buf
local ffi = require 'ffi'
local istype = ffi.istype
local sizeof = ffi.sizeof
local cbor = require 'cbor'
local defer = require 'defer'

local function wrap(fn)
  return coroutine.wrap(function ()
    local success, result = xpcall(fn, debug.traceback)
    if not success then
      print(result)
    end
  end)
end

return function (send, storage_path)

  -- coroutines waiting on a sync to finish
  local waiting = {}

  local function store_chunk(hash, index, data)
    assert(type(hash) == 'string')
    assert(type(index) == 'number')
    if istype(buf, data) then
      data = ffi.string(data, sizeof(data))
    end
    assert(type(data) == 'string')
    assert(fs.writeFile(join(storage_path, hash, string.format('%x', index)), data, true))
  end

  local function store_meta(hash, num_chunks)
    assert(type(hash) == 'string')
    assert(type(num_chunks) == 'number')
    local data = cbor.encode {
      num_chunks = num_chunks,
    }
    local meta_path = join(storage_path, hash, 'meta')
    local temp_path = join(storage_path, hash, '.meta.tmp')
    assert(fs.writeFile(temp_path, data, true))
    assert(fs.rename(temp_path, meta_path))
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
    local data, error = fs.readFile(meta_path)
    if not data then return nil, error end
    local message = cbor.decode(data)
    return message.num_chunks
  end

  -- check what chunks are missing.  Also store_meta
  local function local_sync(hash, num_chunks)
    print('local_sync', hash, num_chunks)

    if num_chunks then
      store_meta(hash, num_chunks)
    else
      num_chunks = load_meta(hash)
      if not num_chunks then return false, 0 end
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
    if #ranges > 0 then
      store_meta(hash, num_chunks, ranges[#ranges])
      return false, unpack(ranges)
    end

    print("Deferring")
    defer(function ()
      print("Deferred")
      local list = waiting[hash]
      if list then
        waiting[hash] = nil
        for i = 1, #list do
          print('resuming coroutine')
          coroutine.resume(list[i], num_chunks)
        end
      end
    end)

    return true, num_chunks
  end

  local downloads = {}
  local paused = false

  local function start_push(hash, ...)
    -- Request a download to start
    downloads[hash] = {...}
    if paused then
      local thread = paused
      paused = nil
      coroutine.resume(thread)
    end
  end

  local function stop_push(hash)
    -- Request a download to stop
    downloads[hash] = nil
  end

  -- create temporary folder for chunks
  -- stream copy file from mutable space to immutable space
  -- move folder to hash of contents
  local function local_import(path)
    local temp_path, input, output, hash, index
    local mode
    local success, message = xpcall(function ()
      mode = assert(fs.stat(path)).mode

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
    return hash, index, mode
  end

  -- combine chunks and write to target path
  local function local_export(hash, path, mode)
    assert(type(hash) == 'string')
    assert(type(path) == 'string')
    if mode then
      assert(type(mode) == 'number')
    else
      mode = 0x1a4 -- 0o644
    end
    local synced, num_chunks = local_sync(hash)
    if not synced then
      print("Export is waiting...")
      local list = waiting[hash]
      if not list then
        list = {}
        waiting[hash] = list
      end
      list[#list + 1] = coroutine.running()
      num_chunks = coroutine.yield()
      print("Export is resuming...")
    end
    local output = assert(fs.open(path, 'w', mode))
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

  local calls = {}
  local function remote_call(command, ...)
    local co = assert(coroutine.running(), 'Should be run in a coroutine')
    local channel_id = (os.time() + uv.hrtime()) % 0x100000
    send(channel_id, command, ...)
    calls[channel_id] = co
    local res = { assert(coroutine.yield()) }
    return unpack(res, 2)
  end

  local remote_procedures = {
    import = local_import,
    export = local_export,
  }

  local receive_timeouts = {}

  local function on_message(message)
    assert(type(message) == 'table' and #message > 0)

    local first_type = type(message[1])

    -- { hash, ... } - CHUNK, SYNC, ACK, or NAK
    if first_type == 'string' then
      local hash = message[1]
      local next_type = type(message[2])

      -- { hash } - SYNC - asking for num_chunks
      if next_type == 'nil' then
        return send(hash, local_sync(hash))
      end

      -- { hash, number, ... } - SYNC or CHUNK
      if next_type == 'number' then
        local chunk = message[3]

        -- { hash, chunk_index, data } - CHUNK
        if chunk then
          local chunk_index = message[2]
          store_chunk(hash, chunk_index, chunk)
          local timer = receive_timeouts[hash]
          if not timer then
            timer = uv.new_timer()
            receive_timeouts[hash] = timer
            timer:start(1000, 1000, wrap(function ()
              send(hash, local_sync(hash))
              timer:close()
              receive_timeouts[hash] = nil
            end))
          else
            timer:again()
          end
          return
        end

        -- { hash, num_chunks } - SYNC - setting num_chunks
        local num_chunks = message[2]
        return send(hash, local_sync(hash, num_chunks))

      end

      -- { hash, boolean, ... } - ACK or NAK
      if next_type == 'boolean' then


        -- { hash, true, num_chunks } - ACK
        if message[2] then
          return stop_push(hash)
        end

        -- { hash, false, 1, 4, 6, 7 } - NAK with ranges
        return start_push(hash, unpack(message, 3))
      end

    end

    -- { channel_id, ... } - Remote procedure calls or replies
    if first_type == 'number' then
      local channel_id = message[1]
      local next_type = type(message[2])

      -- { channel_id, "export", hash, path, mode } -- mode is optional
      -- { channel_id, "import", path }, --> returns file hash, num_chunks
      if next_type == 'string' then
        local fn = remote_procedures[message[2]]
        assert(type(fn) == 'function', 'No such remote procedure')
        return coroutine.wrap(function ()
          local success, result = xpcall(function ()
            return { fn(unpack(message, 3)) }
          end, debug.traceback)
          if success then
            send(channel_id, true, unpack(result))
          else
            print(result)
            send(channel_id, false, result:match("[^\n]+"))
          end
        end)()
      end

      -- { channel_id, boolean, ... } - Remote procedure replies
      if next_type == 'boolean' then
        local co = assert(calls[channel_id])

        -- { channel_id, true, ...return_values },
        -- { channel_id, false, error_message},
        return coroutine.resume(co, unpack(message, 2))

      end

    end

    error 'Invalid message!'
  end

  local function call_export(...)
    return remote_call('export', ...)
  end

  local function call_import(...)
    return remote_call('import', ...)
  end

  local function send_chunk(hash, index, chunk)
    if type(chunk) == 'string' then
      chunk = bin(chunk)
    end
    send(hash, index, chunk)
  end

  local function send_sync(hash, ...)
    assert(type(hash) == 'string')
    local num_chunks = ...
    if num_chunks then
      assert(type(num_chunks) == 'number')
    end
    return send(hash, ...)
  end

  local function send_ack(hash, num_chunks)
    assert(type(hash) == 'string')
    assert(type(num_chunks) == 'number')
    return send(hash, true, num_chunks)
  end

  local function send_nak(hash, ...)
    assert(type(hash) == 'string')
    return send(hash, false, ...)
  end

  coroutine.wrap(function ()
    local next_hash
    while true do
      local success, message = xpcall(function ()
        while true do
          local hash = next(downloads, next_hash)
          if not (hash or next_hash) then break end
          next_hash = hash
          if hash then
            local ranges = downloads[hash]
            if #ranges < 2 then
              downloads[hash] = nil
              return
            end
            local first = ranges[1]
            local last = ranges[2]
            send_chunk(hash, first, load_chunk(hash, first))
            ranges[1] = first + 1
            if first + 1 == last then
              if #ranges == 2 then
                downloads[hash] = nil
              else
                downloads[hash] = { unpack(ranges, 3) }
              end
            end
            return
          end
        end
        print "Pausing..."
        paused = coroutine.running()
        coroutine.yield()
        print "Unpausing..."
      end, debug.traceback)
      if not success then
        print(message)
      end
    end
  end)()

  return {
    on_message = on_message,
    store_meta = store_meta,
    local_import = local_import,
    local_export = local_export,
    local_sync = local_sync,
    call_export = call_export,
    call_import = call_import,
    send_chunk = send_chunk,
    send_sync = send_sync,
    send_ack = send_ack,
    send_nak = send_nak,
  }
end
