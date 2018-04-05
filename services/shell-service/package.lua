return {
  name = "kubos/kubos-shell-service",
  version = "0.0.6",
  description = "Service to provide access to linux files and processes remotely.",
  tags = { "kubos", "udp", "shell", "file", "process" },
  author = { name = "Tim Caswell", email = "tim@kubos.co" },
  homepage = "https://github.com/kubos/kubos",
  luvi = {
    flavor = "tiny",
    inline = "#!/home/system/usr/bin/luvi-tiny --\n"
  },
  dependencies = {
    "luvit/require",
    "luvit/pretty-print",
    "creationix/cbor",
  },
  files = {
    "**.lua",
    "!tests",
  },
  license = "Apache 2.0"
}
