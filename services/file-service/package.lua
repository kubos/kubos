return {
  name = "kubos/kubos-file-service",
  version = "1.0.0",
  description = "Service to upload and download files to cubesats.",
  tags = { "kubos", "udp", "file" },
  author = { name = "Tim Caswell", email = "tim@kubos.co" },
  homepage = "https://github.com/kubos/kubos",
  luvi = {
    flavor = "tiny",
    inline = "#!/home/system/usr/bin/luvi-tiny --\n"
  },
  dependencies = {
    "luvit/require",
    "luvit/pretty-print",
    "kubos/cbor-message-protocol",
    "kubos/file-protocol",
  },
  files = {
    "**.lua",
    "!tests",
  },
  license = "Apache 2.0"
}
