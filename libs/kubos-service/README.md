# Python Service Library

This library simplifies the process of creating a Kubos Service in Python.

It provides the ability to read the service's IP information from the config file
and then starts up GraphQL and Graphiql endpoints at the requested address

More information about developing services can be found in our
[service development guide](https://docs.kubos.com/latest/ecosystem/services/service-dev.html).

# Use

The `Config` class, contained in the `config.py` file, provides the ability to
easily load the configuration information for the service from the provided
configuration file.

The `http_service` function, contained in the `http_service.py` file,
spawns a Flask-based HTTP service with two endpoints:

- `http://{ip}:{port}/` - The default interface for GraphQL requests from
  applications and other entities
- `http://{ip}:{port}/graphiql` - A graphical interface for manual GraphQL
  requests from the user via a web browser 

## Configuration

Services which use this library have the option of using a local configuration file
or falling back on default config values. The service will search for the configuration
file at this location `/home/system/etc/config.toml` unless otherwise specified with
the `-c` flag at run time.

The service configuration file uses the Toml format and is expected to use the
following layout:

```toml,ignore
[service-name]
config-key = "value"
config-key2 = 123

# This section and values are needed for all services
[service-name.addr]
ip = "127.0.0.1"
port = 8082
```

The `[service-name.addr]` section is required for all services and is used to set
the ip/port on which the service will listen for messages. Any service specific
configuration values can be specified directly under the `[service-name]` section.
Note - the `service-name` used in the sections must match the name used when creating
the `Config` instance inside your service.

### Examples

# Creating and starting a simple service.

```python
#!/usr/bin/env python3
from service import schema
from kubos_service import http_service
from kubos_service.config import Config

config = Config("example-service")

# Start an HTTP service
http_service.start(config, schema.schema)
```

# Using the service config info to configure the subsystem.

```python
#!/usr/bin/env python3
import sys

from service import schema
from kubos_service import http_service
from kubos_service.config import Config

c = Config("example-service")

# Set which modules are present and their addresses from the config file.
schema.MODULES = c.raw['modules']

# Starts the HTTP service
http_service.start(c, schema.schema)
```

# Running a service with the default config file (`/home/system/etc/config.toml`).

```bash
$ ./example-service.py
```

# Running a service with a custom config file.

```bash
$ ./example-service.py -c config.toml
```