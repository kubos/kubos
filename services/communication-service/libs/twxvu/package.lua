return {
  name = "kubos/serial-twxvu",
  version = "0.0.1",
  description = "Serial drivers for twxvu radio",
  tags = { "kubos", "radio", "twxvu" },
  author = { name = "Tim Caswell", email = "tim@kubos.co" },
  homepage = "https://github.com/kubos/kubos",
  dependencies = {
    "luvit/require",
    "luvit/pretty-print",
  },
  files = {
    "**.lua",
    "**.h",
    "!test*",
    "$OS-$ARCH/*",
    "$OS-arm/*",
  }
}
