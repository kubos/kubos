-- This transport is for communicating over a serial device.  Simply point it
-- to `/dev/ttyUSB*` or `/dev/ttyUSBO*` or whatever device you want with the
-- agreed upon baud rate and it will open the device file and configure it.

local uv = require 'uv'
local new_radio = require 'nsl-serial'

local function sleep(ms)
  local thread = coroutine.running()
  local timer = uv.new_timer()
  timer:start(ms, 0, function ()
    coroutine.resume(thread)
  end)
  coroutine.yield()
  timer:close()
end

return function (dev, baud)
  assert(dev, 'missing device argument to serial transport')
  assert(baud, 'missing baud argument to serial transport')
  baud = assert(tonumber(baud), 'baud is not a number')

  local radio = new_radio(dev, baud)
  local get_uploaded_file_count = radio.get_uploaded_file_count
  local get_uploaded_file = radio.get_uploaded_file
  local get_download_file_count = radio.get_download_file_count
  local put_download_file = radio.put_download_file

  -- coroutine.wrap(function ()
  --   p("alive", radio.get_alive())
  --   p("uploaded_file_count", get_uploaded_file_count())
  --   -- p("uploaded_file", radio.get_uploaded_file())
  --   p("uploaded_message_count", radio.get_uploaded_message_count())
  --   p("state_of_health_for_modem", radio.get_state_of_health_for_modem())
  --   p("download_file_count", radio.get_download_file_count())
  --   p("geolocation_position_estimate", radio.get_geolocation_position_estimate())
  -- end)()

  print 'Nearspace serial transport setup:'
  p {
    dev = dev,
    baud = baud,
  }

  local function read()
    repeat
      local count = get_uploaded_file_count()
      if count == 0 then sleep(1000) end
    until count > 0
    local name, body = get_uploaded_file()
    p("Received file", name)
    return body
  end

  local count = 1
  local function write(data)
    -- while get_download_file_count() > 1 do
    --   sleep(1000)
    -- end
    local name = 'p' .. count
    count = count + 1
    p("Sending file", name)
    assert(put_download_file(name, data))
  end
  return read, write
end
