local action_frame = require 'action-frame'
local encode = action_frame.encode
local decode = action_frame.decode

local gateways = {}

local function wrap(read, write)
  -- accepts { source = ..., dest = ..., data = ...}
  local function send(packet)
    if not packet then return write() end
    return write {
      opcode = 1,
      payload = encode(packet),
    }
  end

  -- returns { source = ..., dest = ..., data = ..., checksum = ... }
  local function receive()
    while true do
      local message = read()
      if not message then return end
      if message.opcode == 1 then
        return decode(message.payload)
      end
    end
  end

  return receive, send
end

local function new_gateway(gateway_id, read, write)

  local receive, send = wrap(read, write)

  gateways[gateway_id] = { receive, send }

  return receive, send
end

local function get_gateway(gateway_id)
  return gateways[gateway_id]
end

return {
  wrap = wrap,
  new = new_gateway,
  get = get_gateway,
}
