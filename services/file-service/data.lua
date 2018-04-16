
-----------------------------

return {
  { hash, chunk_index, data }, -- send chunk no reply needed
  { hash, chunk_len }, -- syn
  { hash, true }, -- ack
  { hash, false, 1, 4, 6, 7 }, -- nak
  { channel_id, "export", {
    hash = hash,
    name = "big-file.bin",
    path = "/var/data/", -- optional
    mode = 0x180, -- optional
    uid = 'root', -- optional
    gid = 0, -- optional
  } },
  { channel_id, "import", path }, --> returns file hash, chunk_len
  { channel_id, true, value },
  { channel_id, false, error_message},
  -- TODO: add scandir, mkdir, copy, move, delete, etc...
}
