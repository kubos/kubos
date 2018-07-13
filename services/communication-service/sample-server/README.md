To run this sample server:

```sh
lit install
luvit server.lua
```

Then in the communication service on the same server, you need the following
config to connect to this server:

```toml
[[communication-service]]
name = "Local UDP Test Services"
transport = "udp"

[[communication-service]]
name = "Sample Websocket Server"
transport = "action-cable"
url = "wss://localhost:8443/register"
token = PASTE IN TOKEN FROM SERVER OUTPUT
```
