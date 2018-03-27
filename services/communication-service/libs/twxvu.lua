local ffi = require 'ffi'

ffi.cdef((module:load("./twxvu.h")))
local K = ffi.load("/home/system/usr/local/lib/libtwxvu.so")

local rate_table = {
  [1200] = K.RADIO_TX_RATE_1200,
  [2400] = K.RADIO_TX_RATE_2400,
  [4800] = K.RADIO_TX_RATE_4800,
  [9600] = K.RADIO_TX_RATE_9600,
}

local idle_table = {
  [false] = K.RADIO_IDLE_OFF,
  [true]  = K.RADIO_IDLE_ON,
}

local reset_table = {
  hard = K.RADIO_HARD_RESET,
  soft = K.RADIO_SOFT_RESET
}

local function check_status(status, value, empty)
  if status == K.RADIO_OK then
    return value or 1
  end
  if status == K.RADIO_RX_EMPTY then
    return empty or 0
  end
  if status == K.RADIO_ERROR then
    return nil, "RADIO_ERROR"
  end
  if status == K.RADIO_ERROR_CONFIG then
    return nil, "RADIO_ERROR_CONFIG"
  end
end

-- handle to prevent gc of latest msg value
local beacon_msg

local function radio_init()
  return check_status(K.k_radio_init())
end

local function radio_terminate()
  return K.k_radio_terminate()
end

local function radio_configure(options)
  local config = ffi.new("radio_config")

  config.data_rate = assert(
    rate_table[options.data_rate],
    "Invalid `data_rate` (must be 1200, 2400, 4800, or 9600)"
  )

  config.idle = assert(
    idle_table[options.idle] or options.idle == nil and K.RADIO_IDLE_UNKNOWN,
    "Invalid `idle` (must be true, false or nil)"
  )

  local beacon = assert(options.beacon, "Missing `beacon` option")
  assert(type(beacon) == 'table', "`beacon` must be table")
  local interval = assert(beacon.interval, "Missing `beacon.interval`")
  assert(type(interval) == 'number', "`beacon.interval` must be number")
  local msg = assert(beacon.msg, "Missing `beacon.msg`")
  assert(type(msg) == 'string', '`beacon.msg` must be number')
  local len = #msg
  beacon_msg = ffi.new("char[?]", len, msg)
  config.beacon.interval = interval
  config.beacon.msg = beacon_msg
  config.beacon.len = #msg

  local to = options.to
  assert(type(to) == 'table', '`to` must be table')
  local ascii = assert(to.ascii, 'Missing `to.ascii`')
  assert(type(ascii) == 'string', '`to.ascii` must be string')
  local ssid = assert(to.ssid, 'Missing `to.ssid`')
  assert(type(ssid) == 'number', '`to.ssid` must be number')
  ffi.copy(config.to.ascii, ascii, 6)
  config.to.ssid = ssid

  local from = options.from
  assert(type(from) == 'table', '`from` must be table')
  ascii = assert(from.ascii, 'Missing `from.ascii`')
  assert(type(ascii) == 'string', '`from.ascii` must be string')
  ssid = assert(from.ssid, 'Missing `from.ssid`')
  assert(type(ssid) == 'number', '`from.ssid` must be number')
  ffi.copy(config.from.ascii, ascii, 6)
  config.from.ssid = ssid

  return check_status(K.k_radio_configure(config))
end

local function radio_reset(type)
  local ktype = assert(reset_table[type], "type must be 'hard' or 'soft'")
  return check_status(K.k_radio_reset(ktype))
end

local function radio_recv()
  local buffer = ffi.new 'radio_rx_message'
  local len = ffi.new 'uint8_t[1]'
  local status = K.k_radio_recv(buffer, len)
  local result
  if status == K.RADIO_OK then
    result = {
      doppler = buffer.doppler_offset,
      strength = buffer.signal_strength,
      msg = ffi.string(buffer.message, buffer.msg_size),
    }
  end

  return check_status(status, result, {})
end

local function radio_send(data)
  local len = #data
  local buffer = ffi.new('char[?]', len, data)
  local response = ffi.new 'uint8_t[1]'
  local status = K.k_radio_send(buffer, len, response)
  return check_status(status, response[0])
end

return {
  init = radio_init,
  terminate = radio_terminate,
  configure = radio_configure,
  reset = radio_reset,
  send = radio_send,
  recv = radio_recv
}
