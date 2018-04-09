local bit = require 'bit'
local bor = bit.bor
local lshift = bit.lshift
local rshift = bit.rshift
local band = bit.band
local bxor = bit.bxor
local byte = string.byte
local match = string.match
local format = string.format
local char = string.char
local os = require 'os'

local safe_serial = require 'safe-serial'

-- http://mdfs.net/Info/Comp/Comms/CRC16.htm
local function xmodem_crc16(data)
  local crc = 0
  -- Step through bytes in memory
  for i = 1, #data do
    -- Fetch byte from memory, XOR into CRC top byte
    crc = bxor(crc, lshift(byte(data, i), 8))
    for _ = 1, 8 do
      -- rotate
      crc = lshift(crc, 1)
      -- bit 15 was set (now bit 16)...
      if band(crc, 0x10000) > 0 then
        -- XOR with XMODEM polynomic
        -- and ensure CRC remains 16-bit value
        crc = band(bxor(crc, 0x1021), 0xffff)
      end
    end
  end
  return crc
end

return function (dev, baud)
  local serial_read, serial_write, protect = safe_serial(dev, baud)

  local function sync()
    repeat
      -- repeat
      --   local first = serial_read(1)
      -- until first == 'G'
      local second = serial_read(1)
    until second == 'U'
  end

  local function be_u32()
    local u32 = serial_read(4)
    return bor(
      lshift(byte(u32, 1), 24),
      lshift(byte(u32, 2), 16),
      lshift(byte(u32, 3), 8),
      byte(u32, 4))
  end

  local function be_u16()
    local u16 = serial_read(2)
    return bor(
      lshift(byte(u16, 1), 8),
      byte(u16, 2))
  end

  local function u8()
    return byte(serial_read(1), 1)
  end

  local function ack_or_nak()
    sync()
    local r = byte(serial_read(1), 1)
    assert(r == 0x06 or r == 0x0f)
    return r == 0x06
  end

  local function get_state_of_health_for_modem()
    serial_write 'GUGETSOH'
    sync()
    return {
      -- Current epoch reset count, starts at 0, incremented for each power
      -- system reset, persistent over the life of the mission.
      reset_count = be_u32(),
      -- Current time (seconds) from start of most recent reset.
      current_time = be_u32(),
      -- Current RSSI (Received Signal Strength Indicator), 0 to 4
      current_rssi = u8(),
      -- Connection status, 0 (connected) or 1 (disconnected)
      connection_status = u8(),
      -- Globalstar gateway connected to, proprietary ID, 0 to 255
      globalstar_gateway = u8(),
      -- Last contact time, seconds since latest reset
      last_contact_time = be_u32(),
      -- Last attempt time, seconds since latest reset
      last_attempt_time = be_u32(),
      -- Count of call attempts since latest reset
      call_attempts_since_reset = be_u32(),
      -- Count of successful connects since latest reset
      successful_connects_since_reset = be_u32(),
      -- Average connection duration (seconds)
      average_connection_duration = be_u32(),
      -- Connection duration standard deviation (seconds)
      connection_duration_std_dev = be_u32(),
    }
  end

  local function get_uploaded_file_count()
    serial_write 'GUGETUFC'
    sync()
    return be_u32()
  end

  local function get_uploaded_message_count()
    serial_write 'GUGETUMC'
    sync()
    return be_u32()
  end

  local function get_download_file_count()
    serial_write 'GUGETDFC'
    sync()
    return be_u32()
  end

  local function get_geolocation_position_estimate()
    serial_write 'GUGETGEO'
    sync()
    local units = { km = 1000, m = 1 }
    local offset = os.time() - os.time(os.date '!*t')
    local record = serial_read(80)
    local n1, n2, n3 = assert(match(record, 'N: (%d+) (%d+) (%d+)'))
    local w1, w2, w3 = assert(match(record, 'W: (%d+) (%d+) (%d+)'))
    local d, m, y, H, M, S = assert(match(record, 'TIME: (%d+) (%d+) (%d+) (%d+):(%d+):(%d+)'))
    local e, u = assert(match(record, 'ERR: < (%d+) (%w+)'))
    return {
      lon = -(tonumber(w1) + tonumber(w2) / 60 + tonumber(w3) / 3600.0),
      lat = tonumber(n1) + tonumber(n2) / 60 + tonumber(n3) / 3600.0,
      time = os.time {
        day = d, month = m, year = y, hour = H, min = M, sec = S
      } + offset,
      max_error = e * units[u],
    }
  end

  local function get_uploaded_file()
    serial_write 'GUGET_UF'
    sync()
    local name_length = tonumber(assert(serial_read(3)))
    local body_length = tonumber(assert(serial_read(6)))
    local name = assert(serial_read(name_length))
    local body = assert(serial_read(body_length))
    local crc = be_u16()
    -- TODO: verify CRC
    return name, body, crc
  end

  local function delete_download_files()
    serial_write 'GUDLTQDF'
    return be_u32()
  end

  local function delete_uploaded_files()
    serial_write 'GUDLTQUF'
    return be_u32()
  end

  local function delete_uploaded_messages()
    serial_write 'GUDLTQUM'
    return be_u32()
  end

  local function put_download_file(name, body)
    serial_write 'GUPUT_DF'
    assert(ack_or_nak())
    local output = format('GU%03d%06d%s%s', #name, #body, name, body)
    local crc = xmodem_crc16(output)
    output = output .. char(rshift(crc, 8), band(crc, 0xff))
    serial_write(output)
    return ack_or_nak()
  end

  local function get_alive()
    serial_write 'GUGETALV'
    return ack_or_nak()
  end

  return {
    get_state_of_health_for_modem = protect(get_state_of_health_for_modem),
    get_uploaded_file_count = protect(get_uploaded_file_count),
    get_uploaded_message_count = protect(get_uploaded_message_count),
    get_download_file_count = protect(get_download_file_count),
    get_geolocation_position_estimate = protect(get_geolocation_position_estimate),
    get_uploaded_file = protect(get_uploaded_file),
    delete_download_files = protect(delete_download_files),
    delete_uploaded_files = protect(delete_uploaded_files),
    delete_uploaded_messages = protect(delete_uploaded_messages),
    put_download_file = protect(put_download_file),
    get_alive = protect(get_alive),
  }
end
