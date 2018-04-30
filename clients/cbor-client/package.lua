  return {
    name = "kubos/kubos-cbor-client",
    version = "1.1.0",
    description = "Client for CBOR UDP services.",
    tags = { "kubos", "udp", "cbor", "readline"},
    author = { name = "Tim Caswell", email = "tim@kubos.co" },
    homepage = "https://github.com/kubos/kubos",
    luvi = {
      flavor = "tiny",
      inline = "#!/home/system/usr/bin/luvi-tiny --\n"
    },
    dependencies = {
      "luvit/require",
      "luvit/pretty-print",
      "luvit/readline",
      "creationix/cbor",
    },
    files = {
      "**.lua",
    },
    license = "Apache 2.0"
  }
