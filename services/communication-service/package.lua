return {
  name = "creationix/kubos-communication-service",
  version = "0.0.2",
  description = "Service to route udp packets to and from a custom transport.",
  tags = { "kubos", "udp", "nat", "stdio", "serial" },
  author = { name = "Tim Caswell", email = "tim@creationix.com" },
  homepage = "https://github.com/kubos/kubos",
  luvi = {
    flavor = "tiny",
    inline = "#!/home/system/usr/local/bin/luvi-tiny --\n"
  },
  dependencies = {
    "luvit/require",
    "luvit/pretty-print",
    "creationix/coro-wrapper",
    "creationix/coro-channel",
    "creationix/coro-fs",
    "creationix/coro-net",
  },
  files = {
    "**.lua",
    "!libs/twxvu"
  }
}
