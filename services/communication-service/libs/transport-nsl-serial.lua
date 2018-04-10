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
  local get_uploaded_message_count = radio.get_uploaded_message_count
  local get_uploaded_message = radio.get_uploaded_message
  local get_download_file_count = radio.get_download_file_count
  local put_download_file = radio.put_download_file
  local get_state_of_health_for_modem = radio.get_state_of_health_for_modem

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
    while true do
      local ufile = get_uploaded_file_count()
      p("Upload File Count", ufile)
      if ufile > 0 then
        local name, body = get_uploaded_file()
        p("Received file", name)
        return body
      end
      local umessage = get_uploaded_message_count()
      p("Upload Message Count", umessage)
      if umessage > 0 then
        p("message", get_uploaded_message())
      else
        local dfile = get_download_file_count()
        p("Download File Count", dfile)
        local health = get_state_of_health_for_modem()
        p(health)
        sleep(5000)
      end
    end
  end

  local count = 1
  local function write(data)
    p("Download count", get_download_file_count())
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
