local uv = require 'uv'
local cbor = require 'cbor'

-- default lua strings to utf8 strings in cbor encoding
cbor.type_encoders.string = cbor.type_encoders.utf8string

local ffi = require('ffi')
-- Define the bits of the system API we need.
ffi.cdef[[
  int open(const char *pathname, int flags);
  static const int O_RDWR = 2;
  char *ptsname(int fd);
  int grantpt(int fd);
  int unlockpt(int fd);
  struct winsize {
      uint8_t ws_row;
      uint8_t ws_col;
      uint8_t ws_xpixel;
      uint8_t ws_ypixel;
  };
  static const int TIOCSWINSZ = 0x5414;
  int ioctl(int fd, unsigned long request, ...);
  static const int SIGWINCH = 28;
]]

local C = ffi.C

local function openpty()
  local master = C.open("/dev/ptmx", C.O_RDWR)
  assert(master > 0, "Problem opening master for pty")
  assert(C.grantpt(master) == 0, "Problem granting slave pts")
  assert(C.unlockpt(master) == 0, "Problem unlocking slave pts")
  local slave_path = ffi.string(C.ptsname(master))
  local slave = C.open(slave_path, C.O_RDWR)
  assert(slave > 0, "Problem opening slave for pty")
  return master, slave
end

local server = uv.new_udp()
local gateway -- address to write responses to.

local processes = {} -- map of request id to process metadata

-- Helper to send a UDP message down to ground
local function send(...)
  p("->", {...})
  local message = cbor.encode{...}
  server:send(message, gateway.ip, gateway.port)
end

-- Takes in path and options, outputs pid
-- emits exit, stdout, and stderr events
local function spawn(id, path, options)
  if not options then options = {} end

  local process = {}
  processes[id] = process

  local handle, pid, on_stdout, on_stderr, on_exit

  local function check_done()
    -- If the process is exited and both output streams are closed, release the
    -- reference in the global map.
    if (process.code or process.signal)
    and not (process.stdout or process.stderr) then
      processes[id] = nil
    end
  end

  function on_exit (code, signal)
    process.code = code
    process.signal = signal
    send("s-exit", code, signal)
    check_done()
  end

  function on_stdout (err, data)
    if err then return print(err) end
    send("s-out", data)
    if not data then process.stdout = nil end
    check_done()
  end

  function on_stderr (err, data)
    if err then return print(err) end
    send("s-err", data)
    if not data then process.stderr = nil end
    check_done()
  end

  local stdin, stdout, stderr, master, slave

  if options.pty then
    master, slave = openpty()
    local pipe = uv.new_pipe(false)
    pipe:open(master)
    stdin = pipe
    stdout = pipe
    stderr = nil
    options.stdio = {slave, slave, slave}
  else
    stdin = uv.new_pipe(false)
    stdout = uv.new_pipe(false)
    stderr = uv.new_pipe(false)
    options.stdio = {stdin, stdout, stderr}
  end

  handle, pid = assert(uv.spawn(path, options, on_exit))
  stdout:read_start(on_stdout)
  if stderr then
    stderr:read_start(on_stderr)
  end

  process.master = master
  process.slave = slave
  process.handle = handle
  process.pid = pid
  process.stdin = stdin
  process.stdout = stdout
  process.stderr = stderr
  send("s-pid", pid)
end

local function spawn_in(id, data)
  local process = assert(processes[id], "bad process id")
  assert(process.stdin, "can't writing to closed stdin")
  if data then
    process.stdin:write(data)
  else
    local stdin = process.stdin
    process.stdin = nil
    stdin:shutdown(function () stdin:close() end)
  end
end

local function spawn_kill(id, signal)
  local process = assert(processes[id], "bad process id")
  process.handle:kill(signal or 15)
end

local function spawn_resize(id, cols, rows)
  local process = assert(processes[id], "bad process id")
  local winp = ffi.new("struct winsize")
  winp.ws_row = rows
  winp.ws_col = cols
  if C.ioctl(process.master, C.TIOCSWINSZ, winp) ~= 0 then
    print "Error setting window size"
  end
  process.handle:kill(C.SIGWINCH)
end

local commands = {
  spawn = spawn,
  ["s-in"] = spawn_in,
  ["s-kill"] = spawn_kill,
  ["s-resize"] = spawn_resize
}

assert(server:bind("127.0.0.1", 6000))

server:recv_start(function (err, data, addr)
  if err then return print(err) end
  if not data then return end
  gateway = addr
  local success, error = xpcall(function ()
    local message = cbor.decode(data)
    p("<-", message)
    assert(type(message) == "table" and #message >= 1, "Message must be list")
    local command = table.remove(message, 1)
    local fn = commands[command]
    assert(type(fn) == "function", "Invalid message")
    fn(addr.port, unpack(message))
  end, debug.traceback)
  if not success then
    print(error)
  end
end)


-- Workaround for bug in published version of luvi where method is `bindgetsockname`
p("UDP server bound", server.getsockname and server:getsockname() or server:bindgetsockname())

uv.run()
