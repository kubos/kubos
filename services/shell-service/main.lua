--[[
Copyright (C) 2018 Kubos Corporation

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
]]

-- Message format: See README for more details
-- { channel, message, ...args }

local uv = require 'uv'

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
  int kill(int32_t pid, int sig);
]]

local C = ffi.C

local function openpty()
  local master = C.open('/dev/ptmx', C.O_RDWR)
  assert(master > 0, 'Problem opening master for pty')
  assert(C.grantpt(master) == 0, 'Problem granting slave pts')
  assert(C.unlockpt(master) == 0, 'Problem unlocking slave pts')
  local slave_path = ffi.string(C.ptsname(master))
  local slave = C.open(slave_path, C.O_RDWR)
  assert(slave > 0, 'Problem opening slave for pty')
  return master, slave
end

local processes = {}

local Process = {}

function Process:list()
  local out = {}
  for k, v in pairs(processes) do
    if C.kill(v.pid, 0) == 0 then
      out[k] = v
    end
  end
  processes = out
  self:send('list', processes)
end

-- Takes in path and options, outputs pid
-- emits exit, stdout, and stderr events
function Process:spawn(path, options)

  if not options then options = {} end

  local handle, pid, on_stdout, on_stderr, on_exit

  function on_exit (code, signal)
    self.code = code
    self.signal = signal
    self:send('exit', code, signal)
    processes[self.id] = nil
  end

  function on_stdout (err, data)
    if err then return print(err) end
    self:send('stdout', data)
    if not data then self.stdout = nil end
  end

  function on_stderr (err, data)
    if err then return print(err) end
    self:send('stderr', data)
    if not data then self.stderr = nil end
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
  processes[self.id] = {
    path = path,
    pid = pid,
  }
  stdout:read_start(on_stdout)
  if stderr then
    stderr:read_start(on_stderr)
  end

  self.master = master
  self.slave = slave
  self.handle = handle
  self.pid = pid
  self.stdin = stdin
  self.stdout = stdout
  self.stderr = stderr
  self:send('pid', pid)
end

function Process:stdin(data)
  assert(self.stdin, 'Can not write to closed stdin')
  if data then
    self.stdin:write(data)
  else
    local stdin = self.stdin
    self.stdin = nil
    stdin:shutdown(function () stdin:close() end)
  end
end

function Process:kill(signal)
  self.handle:kill(signal or 15)
end

function Process:resize(cols, rows)
  local winp = ffi.new('struct winsize')
  winp.ws_row = rows
  winp.ws_col = cols
  if C.ioctl(self.master, C.TIOCSWINSZ, winp) ~= 0 then
    print 'Error setting window size'
  end
  self.handle:kill(C.SIGWINCH)
end

require('channel-service')(Process, 6000)

uv.run()
