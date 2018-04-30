# Kubos CBOR CLI Tool

This simple tool makes it easy to test any cbor based protocol.

Build and run this the same way as the other [lua clients](../file-client).

## Usage

Simply run `kubos-cbor-client service_port` and send commands encoded as lua.

For example, you can talk to the shell service directly:

```lua
$ kubos-cbor-client 6000
udp:6000> { 42, 'spawn', '/bin/uname', { args = { '-a' } } }
Client: { 42, 'spawn', '/bin/uname', { args = { '-a' } } }
Server: { 42, 'pid', 15682 }
Server: { 42, 'stdout', 'Linux t580 4.16.3-041603-generic #201804190730 SMP Thu Apr 19 07:32:02 UTC 2018 x86_64 x86_64 x86_64 GNU/Linux\n' }
Server: { 42, 'stdout' }
Server: { 42, 'stdout' }
Server: { 42, 'exit', 0, 0 }
udp:6000>
```

Or interact with the file service:

```lua
$ kubos-cbor-client 7000
udp:7000> { 101, 'import', 'README.md' }
Client: { 101, 'import', 'README.md' }
Server: { 101, true, '67364040e456a178f09ea0952650bc1d', 1, 33188 }
udp:7000> {'67364040e456a178f09ea0952650bc1d', false, 0, 1}
Client: { '67364040e456a178f09ea0952650bc1d', false, 0, 1 }
Server: { '67364040e456a178f09ea0952650bc1d', 0, cdata<unsigned char [?]>: 0x405c4a70 }
udp:7000>
```
