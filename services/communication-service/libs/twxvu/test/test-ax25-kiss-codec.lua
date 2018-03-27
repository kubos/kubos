local codec = require 'ax25-kiss-codec'
local encode = codec.encode
local decode = codec.decode

local test1 = '\192\000\150\170\132\158\166\142\224\150\170\132\158\166\166u\003\240ping\192\192\000\150\170\132\158\166\142\224\150\170\132\158\166\166u\003\240Hello world\n\192'
local test2 = "\192\000\150\170\132\158\166\142\224\150\170\132\158\166\166u\003\240I'm sorry Dave, I'm afraid I can't do that\n\192\192\000\150\170\132\158\166\142\224\150\170\132\158\166\166u\003\240ping\192\192\000\150\170\132\158\166\142\224\150\170\132\158\166\166u\003\240Hello Tim\n\192"
local test = test1 .. test2
local i = 1
repeat
  local message
  print()
  p(string.sub(test, i))
  message, i = decode(test, i)
  if message then
    p(message)
    local encoded = encode(message)
    p(encoded)
    p(decode(encoded, 1))
  end
until not i
