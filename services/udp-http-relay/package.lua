return {
  name = "kubos/kubos-udp-http-relay",
  version = "0.0.1",
  description = "Assistant to UDP communication service to expose http services.",
  tags = { "kubos", "udp", "http" },
  author = { name = "Tim Caswell", email = "tim@kubos.co" },
  homepage = "https://github.com/kubos/kubos",
  luvi = {
    flavor = "tiny",
    inline = "#!/home/system/usr/local/bin/luvi-tiny --\n"
  },
  dependencies = {
    "luvit/require",
    "luvit/pretty-print",
    "luvit/http-codec",
    "creationix/coro-http",
  },
  files = {
    "**.lua",
  }
}
