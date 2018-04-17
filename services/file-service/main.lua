local storage = require './.'
local import = storage.import
local export = storage.export
local sync = storage.sync

-- { hash, chunk_index, data }, -- send chunk no reply needed
-- { hash, chunk_len }, -- syn
-- { hash, true }, -- ack
-- { hash, false, 1, 4, 6, 7 }, -- nak
-- { channel_id, "export", {
--   hash = hash,
--   name = "big-file.bin",
--   path = "/var/data/", -- optional
--   mode = 0x180, -- optional
-- } },
-- { channel_id, "import", path }, --> returns file hash, chunk_len
-- { channel_id, true, value },
-- { channel_id, false, error_message},


coroutine.wrap(function ()
  local success, message = xpcall(function ()

    local hash, num_chunks = import("EyeStar-D2_Duplex_ICD_v7.8.pdf")
    p{hash=hash,num_chunks=num_chunks}
    require('coro-fs').unlink('storage/' .. hash .. '/10')
    require('coro-fs').unlink('storage/' .. hash .. '/20')
    require('coro-fs').unlink('storage/' .. hash .. '/30')
    require('coro-fs').unlink('storage/' .. hash .. '/31')
    require('coro-fs').unlink('storage/' .. hash .. '/198')
    require('coro-fs').unlink('storage/' .. hash .. '/199')
    p(sync(hash, num_chunks))
    -- export(hash, "copy.pdf")

  end, debug.traceback)
  if not success then
    print(message)
  end
end)()

require('uv').run()
