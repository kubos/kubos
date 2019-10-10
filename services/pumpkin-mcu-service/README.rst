Pumpkin MCU Service
===================

Hardware service for all Pumpkin Modules that run off MCU commands.

This service listens on 127.0.0.1:8150 for UDP graphql queries and mutations.

Queries are telemetry requests (data obtained from the module)
Mutations are commands (data written to the module)
Both require I2C interaction with the module

.. note::

  The IP address, port, and module address configuration used by this service is controlled by a file `/etc/kubos-config.toml`. You MUST set the module addresses within the config file to match your hardware configuration.

Examples
--------

Example query:

.. code::

  query {
    mcuTelemetry(
      module:"sim",
      fields:["firmware_version","commands_parsed","scpi_errors"]
    )
  }


Example mutation:

.. code::

  mutation {
    passthrough(module:"sim",command:"SUP:LED ON") {
      status,
      command
    }
  }

Some commands to run to test from the command line (for module "sim"):

.. code::

  echo "query {moduleList}" | nc -uw1 127.0.0.1 8150
  echo "query {fieldList(module:\"sim\")}" | nc -uw1 127.0.0.1 8150
  echo "mutation {passthrough(module:\"sim\",command:\"SUP:LED ON\"){status,command}}" | nc -uw1 127.0.0.1 8150

Testing
-------

In the tests folder, there is an integration test script that can be run to verify communication with all modules on the bus. It will retrieve what modules are present, request what fields are available for each module, and retrieve all available telemetry for each module.

Run with:

.. code:: 

  python integration_test.py -c /path/to/service/config.toml
