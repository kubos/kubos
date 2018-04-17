local file_service = require './.'

-- { hash, chunk_index, data }, -- send chunk no reply needed
-- { hash, num_chunks }, -- syn
-- { hash, true, num_chunks }, -- ack
-- { hash, false, num_chunks, 1, 4, 6, 7 }, -- nak
-- { channel_id, "export", hash, path, mode } -- mode is optional
-- { channel_id, "import", path }, --> returns file hash, num_chunks
-- { channel_id, true, value },
-- { channel_id, false, error_message},

local getenv = require('os').getenv
local port = getenv 'PORT'
port = port and tonumber(port) or 7000

require('channel-service')(file_service, port)

require('uv').run()
