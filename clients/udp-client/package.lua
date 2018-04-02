  return {
    name = "kubos/kubos-udp-client",
    version = "0.0.2",
    description = "UDP client for textual UDP services.",
    tags = { "kubos", "udp", "readline"},
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
    },
    files = {
      "**.lua",
    }
  }
