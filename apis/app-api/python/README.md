# Python Application API

The Python application API is meant to simplify development of KubOS mission applications in Python. Currently, it is only a library for easy communication with the hardware services.

## Usage

Usage of the API is shown in the example below

```

    import app_api

    service_api = app_api.Services()

    query_response = service_api.query(
        service = "service-name",
        query = "mutation {noop{success}}")
```

The service accessed by the API must be in the system config file. You can pass in an alternate configuration file, otherwise it will look at the default config file location in KubOS: /etc/kubos-config.toml
