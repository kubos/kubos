# Kubos Communication Service

This service is similar to the popular
[socat](https://linux.die.net/man/1/socat) tool in that it connects two
arbitrary endpoints.  The types of endpoints are known as transports.  These can
be UDP bridges, serial devices, Major Tom integration or various proprietary
flight radio systems.

## Sample Development Configuration

For example, a common dev situation is to use the communication service to bridge
UDP clients on a laptop to UDP services on a dev board.  The dev board would be
configured as followed to expose local services over serial:

```toml
[[communication-service]]
name = "Flight UDP Services"
transport = "udp"

[[communication-service]]
name = "Internal Serial"
transport = "serial"
device = "/dev/ttyO4"
baud = 115200 # Use 9600 to simulate slow radios for testing
```

Then supposing there are two services running on ports `6000` and `7000` we want
to expose them on the host:

```toml
[[communication-service]]
name = "External Serial"
transport = "serial"
device = "/dev/ttyUSB1"
baud = 115200 # Use 9600 to simulate slow radios for testing

[[communication-service]]
name = "Laptop UDP Clients"
transport = "udp"
expose-ports = [ 6000, 7000 ] # Expose the services we want access to.
```

## Building

This is implemented as a luvi service.  See the [shell-service
docs](https://github.com/kubos/kubos/tree/master/services/shell-service#setup)
for details on building and using luvi.

## Running Latest Release

To run the release version, install from lit and run with the path to the config file as only argument.

```sh
lit make lit://kubos/kubos-communication-service
./kubos-communication-service path/to/config.toml
```

If you're using the `debug-serial` transport, make sure to redirect stderr to a
log file:

```sh
kubos-communication-service path/to/config.toml 2> communication-service-log
```

## Running From Source

First enter this folder:

```sh
cd kubos/services/communication-service
```

And install the dependencies locally:

```sh
lit install
```

And run from the file-system directly:

```sh
luvi-tiny . -- path/to/config.toml
```
