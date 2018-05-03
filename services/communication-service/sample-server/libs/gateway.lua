local wrap = require('action-frame').wrap
local JSON = require 'json'

local gateways = {}

local function new_gateway(gateway_id, read, write)

  -- Consume the subscribe message
  assert(JSON.parse(read().payload))

  local receive, send = wrap(read, write)

  gateways[gateway_id] = { receive, send }

  return receive, send
end

local function get_gateway(gateway_id)
  return gateways[gateway_id]
end

return {
  new = new_gateway,
  get = get_gateway,
}
