local ffi = require 'ffi'
local C = ffi.C

ffi.cdef[[
  static const int VMIN = 6;
  static const int VTIME = 5;
  static const int	TCSANOW =		0;
  typedef struct termios {
    unsigned int c_iflag;
    unsigned int c_oflag;
    unsigned int c_cflag;
    unsigned int c_lflag;
    unsigned char c_line;
    unsigned char c_cc[32];
    unsigned int c_ispeed;
    unsigned int c_ospeed;
  } termios_t;
  int tcgetattr(int fd, struct termios *termios_p);
  int tcsetattr(int fd, int optional_actions, const struct termios *termios_p);
  void cfmakeraw(struct termios *termios_p);
  int cfsetspeed(struct termios *termios_p, int speed);
]]

local baud_table = {
  [0] = 0000000,
  [50] = 0000001,
  [75] = 0000002,
  [110] = 0000003,
  [134] = 0000004,
  [150] = 0000005,
  [200] = 0000006,
  [300] = 0000007,
  [600] = 0000010,
  [1200] = 0000011,
  [1800] = 0000012,
  [2400] = 0000013,
  [4800] = 0000014,
  [9600] = 0000015,
  [19200] = 0000016,
  [38400] = 0000017,
  [57600] = 0010001,
  [115200] = 0010002,
  [230400] = 0010003,
  [460800] = 0010004,
  [500000] = 0010005,
  [576000] = 0010006,
  [921600] = 0010007,
  [1000000] = 0010010,
  [1152000] = 0010011,
  [1500000] = 0010012,
  [2000000] = 0010013,
  [2500000] = 0010014,
  [3000000] = 0010015,
  [3500000] = 0010016,
  [4000000] = 0010017,
}

local function set_termio(fd, baud)
  baud = assert(baud_table[baud], "invalid baud rate")
  local tty = ffi.new "termios_t"
  assert(C.tcgetattr(fd, tty) == 0, "Error from tcgetattr")
  C.cfmakeraw(tty)
  assert(C.cfsetspeed(tty, baud) == 0, "error setting baud")
  tty.c_cc[C.VMIN]  = 1 -- wait for at least 1 byte before returning
  -- tty.c_cc[C.VTIME] = 0 -- 0.1 seconds read timeout
  assert(C.tcsetattr (fd, C.TCSANOW, tty) == 0, "Error setting term attributes")
end

return set_termio
