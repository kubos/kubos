-- This transport speaks to the on-flight twxvu radio over it's serial
-- interface.
local radio = require 'twxvu'

return function ()
  assert(radio.init())
  local read = radio.recv
  local write = radio.send

  return read, write
end
