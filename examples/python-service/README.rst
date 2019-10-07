Python Service Example
======================

This is an example of a subsystem service implemented in Python.

This service should be started with the included `config.toml` file, like so::

    $ ./service.py -c config.toml

This service listens on http://0.0.0.0:8123/graphql for
graphql queries and mutations.
Queries are requests for state information (telemetry).
Mutations are equivalent to commands.

There is also a graphiql interface at http://0.0.0.0:8123/graphiql
for ease of development.

.. note::
   The IP address and port used by this service is controlled by a file
   `config.toml` found in the root `python-service` directory.

Currently this payload has a single member `powerOn`.

Example query:

.. code::
   {
       subsystem {
           powerOn
       }
   }


Example mutation:

.. code::
   mutation {
       powerOn(power: true) {
           status
       }
   }
