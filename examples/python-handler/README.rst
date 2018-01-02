Python Handler Example
======================

This is an example of a subsystem handler implemented in Python.

This handler listens on http://127.0.0.1:5000/graphql for
graphql queries and mutations.
Queries are requests for state information (telemetry).
Mutations are equivalent to commands.

There is also a graphiql interface at http://127.0.0.1:5000/graphiql
for ease of development.

.. note::
   The IP address and port used by this handler is controlled by a file
   `config.yml` found in the root `python-handler` directory.

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
       subsystem(powerOn: true) {
           status
       }
   }
