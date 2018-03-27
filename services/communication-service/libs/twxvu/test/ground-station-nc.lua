local split = require 'coro-split'
local codec = require 'ax25-kiss-codec'
local connect = require('coro-net').connect
local stdin = require('pretty-print').stdin
local wrapRead = require('coro-channel').wrapRead

coroutine.wrap(function ()
  print "Connecting to ground station..."
  local read, write = assert(connect {
    family = "inet6",
    host = "fcc0:2961:d391:623f:34ed:a01e:f11e:93af",
    port = 3212,
    encode = codec.encode,
    decode = codec.decode
  })
  print "Connected!"

  split(function ()
    local message = {
      dest = {
        bit = true,
        ssid = 10,
        callsign = "KUBOSS"
      },
      source = {
        bit = false,
        ssid = 0,
        callsign = "KUBOSG"
      }
    }
    print "Enter messages and press enter to send"
    for line in wrapRead(stdin) do
      message.data = line
      write(message)
    end
    print "Stdin exited"
    write()
  end, function ()
    for message in read do
      p(message)
    end
    print "radio exited"
  end)
end)()
