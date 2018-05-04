return {
  name = "kubos/kubos-communication-service",
  version = "0.0.4",
  description = "Service to route udp packets to and from a custom transport.",
  tags = { "kubos", "udp", "nat", "stdio", "serial" },
  author = { name = "Tim Caswell", email = "tim@kubos.co" },
  homepage = "https://github.com/kubos/kubos",
  luvi = {
    flavor = "tiny",
    -- Use luvi-regular for action-cable transport (or anything needing https)
    -- inline = "#!/home/system/usr/bin/luvi-regular --\n"
    -- Use luvi-tiny for anything you want to keep small.
    inline = "#!/home/system/usr/bin/luvi-tiny --\n"
  },
  dependencies = {
    -- Uncomment secure-socket when using luvi-regular
    -- "luvit/secure-socket",
    "luvit/require",
    "luvit/pretty-print",
    "luvit/json",
    "creationix/base64",
    "creationix/coro-wrapper",
    "creationix/coro-channel",
    "creationix/coro-fs",
    "creationix/coro-websocket",
    "creationix/toml",
  },
  files = {
    "**.lua",
    "!sample-server/**",
  },
  license = "Apache 2.0"
}
