-- Message format: See README for more details
-- { channel, message, ...args }

local uv = require 'uv'

local ffi = require('ffi')
-- Define the bits of the system API we need.
ffi.cdef[[
  char *strerror(int errnum);
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
  int kill(int32_t pid, int sig);
]]

local C = ffi.C

local function cassert(val, message)
  if val then return val end
  error(message .. ': ' .. ffi.string(C.strerror(ffi.errno())))
end

local function cwarn(val, message)
  if not val then
    print(message .. ': ' .. ffi.string(C.strerror(ffi.errno())))
  end
  return val
end

local function openpty()
  local master = C.open('/dev/ptmx', C.O_RDWR)
  cassert(master > 0, 'Problem opening master for pty')
  cassert(C.grantpt(master) == 0, 'Problem granting slave pts')
  cassert(C.unlockpt(master) == 0, 'Problem unlocking slave pts')
  local slave_path = ffi.string(C.ptsname(master))
  local slave = C.open(slave_path, C.O_RDWR)
  cassert(slave > 0, 'Problem opening slave for pty')
  return master, slave
end

local processes = {}

-- TODO: find out if we need this sweeper or if it's always clean alrady.
-- local function sweep_dead()
--   p("SWEEP", processes)77
--   local out = {}
--   for k, v in pairs(processes) do
--     if C.kill(v.pid, 0) == 0 then
--       out[k] = v
--     end
--   end
--   processes = out
-- end
--
-- -- Sweep for dead processes on a 10 second interval.
-- local timer = uv.new_timer()
-- timer:start(10000, 10000, sweep_dead)
-- timer:unref()

return function(channel)

  local channel_id = channel.id
  local code, signal
  local master, slave, handle, pid
  local stdin, stdout, stderr
  local service = {}

  function service.list()
    -- sweep_dead()
    channel.list(processes)
  end

  -- Takes in path and options, outputs pid,
  -- emits exit, stdout, and stderr events
  function service.spawn(path, options)
    assert(not handle, 'Process already spawned on this channel')
    assert(type(path) == 'string', 'Path must be a string')
    if not options then options = {} end
    assert(type(options) == 'table', 'Options must be a table')

    local on_stdout, on_stderr, on_exit

    function on_exit (exit_code, exit_signal)
      code = exit_code
      signal = exit_signal
      channel.exit(code, signal)
      processes[channel_id] = nil
    end

    function on_stdout (err, data)
      if err then return print(err) end
      channel.stdout(data)
      if not data then stdout = nil end
    end

    function on_stderr (err, data)
      if err then return print(err) end
      channel.stderr(data)
      if not data then stderr = nil end
    end

    if options.pty then
      assert(type(options.pty) == 'boolean', 'options.pty must be a boolean')
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
    processes[channel_id] = {
      path = path,
      pid = pid,
    }
    stdout:read_start(on_stdout)
    if stderr then
      stderr:read_start(on_stderr)
    end

    master = master
    slave = slave
    handle = handle
    pid = pid
    stdin = stdin
    stdout = stdout
    stderr = stderr
    channel.pid(pid)
  end

  function service.stdin(data)
    assert(handle, 'Need to spawn first before writing to stdin')
    assert(stdin, 'Can not write to closed stdin')
    if data then
      stdin:write(data)
    else
      local copy = stdin
      stdin = nil
      copy:shutdown(function () copy:close() end)
    end
  end

  function service.kill(kill_signal)
    assert(handle, 'Need to spawn first before killing')
    handle:kill(kill_signal or 15)
  end

  local winp = ffi.new('struct winsize')
  function service.resize(cols, rows)
    assert(handle, 'Need to spawn first before resizing')
    assert(master, 'Cannot resize without pty master')
    winp.ws_row = rows
    winp.ws_col = cols
    cwarn(C.ioctl(master, C.TIOCSWINSZ, winp) == 0,
      'Problem setting window size')
    handle:kill(C.SIGWINCH)
  end

  return service
end
