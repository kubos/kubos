  return {
    name = "kubos/kubos-shell-client",
    version = "0.0.4",
    description = "Shell client to connect to remote shell service over custom transport.",
    tags = { "kubos", "udp", "nat", "shell"},
    author = { name = "Tim Caswell", email = "tim@kubos.co" },
    homepage = "https://github.com/kubos/kubos",
    luvi = {
      flavor = "tiny",
      inline = "#!/home/system/usr/local/bin/luvi-tiny --\n"
    },
    dependencies = {
      "luvit/require",
      "luvit/pretty-print",
      "luvit/readline",
      "creationix/cbor",
    },
    files = {
      "**.lua",
    }
  }
