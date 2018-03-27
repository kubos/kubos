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
  [0] = 0,
  [50] = 1,
  [75] = 2,
  [110] = 3,
  [134] = 4,
  [150] = 5,
  [200] = 6,
  [300] = 7,
  [600] = 8,
  [1200] = 9,
  [1800] = 10,
  [2400] = 11,
  [4800] = 12,
  [9600] = 13,
  [19200] = 14,
  [38400] = 15,
  [57600] = 4097,
  [115200] = 4098,
  [230400] = 4099,
  [460800] = 4100,
  [500000] = 4101,
  [576000] = 4102,
  [921600] = 4103,
  [1000000] = 4104,
  [1152000] = 4105,
  [1500000] = 4106,
  [2000000] = 4107,
  [2500000] = 4108,
  [3000000] = 4109,
  [3500000] = 4110,
  [4000000] = 4111,
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
