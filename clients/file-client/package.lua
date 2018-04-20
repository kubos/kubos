return {
  name = "kubos/kubos-file-client",
  version = "0.0.1",
  description = "File client to connect to remote file service over custom transport.",
  tags = { "kubos", "udp", "nat", "file"},
  author = { name = "Tim Caswell", email = "tim@kubos.co" },
  homepage = "https://github.com/kubos/kubos",
  luvi = {
    flavor = "tiny",
    inline = "#!/home/system/usr/bin/luvi-tiny --\n"
  },
  dependencies = {
    "luvit/require",
    "luvit/pretty-print",
    "creationix/pathjoin",
    "kubos/file-protocol",
    "kubos/cbor-message-protocol",
  },
  files = {
    "**.lua",
  }
}
