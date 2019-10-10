Payload Services
================

Payload services are essentially hardware services which have been custom designed
for mission payload hardware. They share the same architecture as the hardware
services, exposing low-level device APIs through a GraphQL interface.

They primarily differ from hardware services in how they are integrated into the final system.
Hardware services each have an accompanying Buildroot package, allowing them to be built into the base operating system image.
This allows them to be covered by the operating system's recovery process.
Payload services are generally intended to have the ability to be easily updated on orbit, so they should be run as processes in the user partition that are started on boot.

The `examples folder of the Kubos repo <https://github.com/kubos/kubos/tree/master/examples>`__ includes
two example payload services: one written in `Python <https://github.com/kubos/kubos/tree/master/examples/python-service>`_,
and one written in `Rust <https://github.com/kubos/kubos/tree/master/examples/rust-service>`__.

.. _python-service-ref:

Python Service
--------------

Inside of the `Python service example <https://github.com/kubos/kubos/tree/master/examples/python-service>`_
you will find several files and a folder:

- ``config.toml`` - This TOML file holds :doc:`configuration options <../services/service-config>`
  for the GraphQL/HTTP endpoint.
- ``README.rst`` - Overview of the service project.
- ``requirements.txt`` - Python module requirements file with list of module/version dependencies.
- ``service.py`` - Boilerplate service code which reads the config file and starts up the GraphQL/HTTP endpoint.
- ``service/`` - This folder holds the guts of the service's source. The service folder contains the main files which will need modifying when building a custom payload service:

    - ``__init__.py`` - This empty file belongs in the `service/` folder to give `service.py` access to the modules within.
    - ``app.py`` - Another boilerplate service file. This one should not require any customization.
    - ``models.py`` - Describes the hardware model exposed to GraphQL and contains calls down into lower level APIs.
    - ``schema.py`` - Contains the actual GraphQL models which are used to generate the GraphQL endpoint.

We will now take a closer look at `models.py` and `schema.py` to see what exactly it takes to expose a hardware
API through the service.

models.py
~~~~~~~~~

Inside of the example `models.py` file there are `Subsystem` and `Status` classes.
Both of these classes must be subclasses of `graphql.ObjectType <http://docs.graphene-python.org/en/latest/types/objecttypes/>`_
from the `Graphene <http://docs.graphene-python.org/en/latest/>`_ module.

The `Subsystem` class models the hardware that this service will be interacting with.

.. code-block:: python

	class Subsystem(graphene.ObjectType):
	    """
	    Model encapsulating subsystem functionality.
	    """

	    power_on = graphene.Boolean()

	    def refresh(self):
		"""
		Will hold code for refreshing the status of the subsystem
		model based on queries to the actual hardware.
		"""

		print "Querying for subsystem status"
		self.power_on = not self.power_on

	    def set_power_on(self, power_on):
		"""
		Controls the power state of the subsystem
		"""

		print "Sending new power state to subsystem"
		print "Previous State: %s" % self.power_on
		print "New State: %s" % power_on
		self.power_on = power_on
		return Status(status=True, subsystem=self)

Member variables can be added if any persistent data needs to be stored.
Member functions are called by the GraphQL schema and are used to call into low-level device API functions.

The `Status` class is used to model any important information gathered from calling device API functions.

.. code-block:: python

	class Status(graphene.ObjectType):
	    """
	    Model representing execution status. This allows us to return
	    the status of the mutation function alongside the state of
	    the model affected.
	    """

	    status = graphene.Boolean()
	    subsystem = graphene.Field(Subsystem)

Right now it just contains a `status` member which represents the status of the function call and a `subsystem` member which represents the current state of the `Subsystem`.

schema.py
~~~~~~~~~

Now lets look inside of `schema.py`. This file contains the models used by `Graphene` to create our GraphQL endpoint.

Queries
^^^^^^^

Queries allow us to fetch data from the subsystem. There is only one `Query` class needed in the `schema.py` file.

.. code-block:: python

	class Query(graphene.ObjectType):
	    """
	    Creates query endpoints exposed by graphene.
	    """

	    subsystem = graphene.Field(Subsystem)

	    def resolve_subsystem(self, info):
		"""
		Handles request for subsystem query.
		"""

		_subsystem.refresh()
		return _subsystem

Any member variables of the type `graphene.Field` become top-level fields accessible by queries.
Because we are using the `Subsystem` class, which is also a `graphene.ObjectType`, members of that class become accessible by queries.
Each Graphene field requires a resolver function named `resolve_fieldname` which returns back an object of the field's class type.
In this case, we call `_subsystem.refresh()` to load the latest data into the global `_subsystem` object and return it.

The above class would enable the following query for subsystem power status::

    {
        subsystem {
            powerOn
        }
    }

Mutations
^^^^^^^^^

Mutations allow us to call functions on the subsystem which cause change or perform some action.
Like the `Query` class we will only need one top level `Mutation` class.

.. code-block:: python

	class Mutation(graphene.ObjectType):
	    """
	    Creates mutation endpoints exposed by Graphene.
	    """

	    power_on = PowerOn.Field()

Like with the `Query`, each `Field` member becomes a top-level mutation.
However for mutations we will create a new class for each mutation field.

.. code-block:: python

	class PowerOn(graphene.Mutation):
	    """
	    Creates mutation for Subsystem.PowerOn
	    """

	    class Arguments:
		power = graphene.Boolean()

	    Output = Status

	    def mutate(self, info, power):
		"""
		Handles request for subsystem powerOn mutation
		"""

		status = Status(status=True, subsystem=_subsystem)
		if power != None:
		    status = _subsystem.set_power_on(power)

		return status

The `Arguments` class describe any argument fields needed for this mutation.
The line ``Output = Status`` describes the class type this mutation should return.
The ``mutate`` function performs the actual work of the mutation and must return back an object of the type specified in the ``Output`` line.
The above classes enable the following mutation::

    mutation {
        powerOn(power:false) {
            status
        }
    }

Running the example
~~~~~~~~~~~~~~~~~~~

Getting the example service up and running is fairly simple.
First, you must make sure you have the necessary Python dependencies installed.
If you are using the Kubos SDK Vagrant box then these will already be installed.
Otherwise, you will need to run ``pip3 install -r requirements.txt``.

Once the dependencies are in place, you can run ``python3 service.py`` and the example service should begin.
You will know that it is running if the command line output says ``* Running on http://127.0.0.1:8123/ (Press CTRL+C to quit)``.
You can now point a web browser to http://127.0.0.1:8123/graphiql to access a `graphical GraphQL interface <https://github.com/graphql/graphiql>`_.
Here you can run queries and mutations against the GraphQL endpoints and see the results.

.. note::

   If you are running the example from within the Vagrant box then you may need
   some additional configuration.

By default the Vagrant box does not forward any ports. In order to access the HTTP
interface of the service running inside of the Vagrant box we need to forward
the port it is using. To do so you will need to add the following line to
your ```Vagrantfile``` (after ``Vagrant.configure("2") do |config|``)::

  config.vm.network "forwarded_port", guest: 8123, host: 8123

Now restart the Vagrant box with ``vagrant reload``. You should now have the ability
to run the python service inside the Vagrant box and access it from the outside
at http://127.0.0.1:8123.

.. _rust-service-ref:

Rust Service
------------

This is a quick overview of the payload service written in Rust.

The current guide for working with Rust within the Kubos SDK can be
found :doc:`here <../../sdk-docs/sdk-rust>`.

Libraries
~~~~~~~~~

This payload service and future rust-based services will be written using
the following external crate:

- `Juniper <https://github.com/graphql-rust/juniper>`__ - GraphQL server library

And one internal helper crate:

- `Kubos Service <../../rust-docs/kubos_service/index.html>`__ - HTTP service interface

The ``Cargo.toml`` in the example payload service gives a good list of crate
dependencies to start with.

Example Source
~~~~~~~~~~~~~~

`Example Source - GitHub <https://github.com/kubos/kubos/tree/master/examples/rust-service>`__

 - ``Cargo.lock`` - Cargo `lock <https://doc.Rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html>`__ file
 - ``Cargo.toml`` - Cargo `manifest <https://doc.Rust-lang.org/cargo/reference/manifest.html>`__ file
 - ``src`` - Contains the actual Rust source.

     - ``main.rs`` - Contains setup code using the ``kubos-service`` crate. May need minor customization but not much.
     - ``model.rs`` - Describes the hardware model exposed to GraphQL and contains calls down to lowel-level APIs.
     - ``schema.rs`` - Contains the actual GraphQL schema models used to generate the GraphQL endpoint.

We will now take a closer look at ``model.rs`` and ``schema.rs`` and break down
the pieces required to expose hardware APIs through the service.

model.rs
~~~~~~~~

The ``model.rs`` file contains structures and functions used to wrap low-level device APIs
and provide abstractions for the GraphQL schema to call into. Looking inside of the ``model.rs``
file you will see several ``struct`` declarations. We'll start with the ``Subsystem``:

.. code-block:: rust

  pub struct Subsystem;

Here we have a struct which is used to model a subsystem. In this example the struct
is given no member variables for persistence. All data is obtained through function
calls for real-time results.

Here is an abbreviated set of functions implemented for the ``Subsystem`` struct:

.. code-block:: rust

	impl Subsystem {
	    /// Creates new Subsystem structure instance
	    /// Code initializing subsystems communications
	    /// would likely be placed here
	    pub fn new() -> Subsystem {
		println!("getting new subsystem data");
		// Here we would call into a hardware API
		Subsystem {}
	    }

	    /// Power status getter
	    /// Code querying for new power value
	    /// could be placed here
	    pub fn power(&self) -> Result<bool, Error> {
		println!("Getting power");
		// Low level query here
		Ok(true)
	    }

	    /// Power state setter
	    /// Here we would call into the low level
	    /// device function
	    pub fn set_power(&self, _power: bool) -> Result<SetPower, Error> {
		println!("Setting power state");
		// Send command to device here
		if _power {
		    Ok(SetPower { power: true })
		} else {
		    Err(Error::new(
		        ErrorKind::PermissionDenied,
		        "I'm sorry Dave, I afraid I can't do that",
		    ))
		}
	    }
	}

	/// Overriding the destructor
	impl Drop for Subsystem {
	    /// Here is where we would clean up
	    /// any subsystem communications stuff
	    fn drop(&mut self) {
		println!("Destructing subsystem");
	    }
	}

The ``new`` function is the ``Subsystem`` constructor. It can be used to establish
a connection with the hardware if necessary. This function is called once per
query or mutation and produces the struct instance used.

The ``power`` function is an example of a function called during a query. These
functions called by GraphQL functions must return the type ``Result<T, Error>``
in order to properly unpack valid data vs an error message.

The ``set_power`` function is an example of a function called during a mutation.
It is essentially the same as ``power`` but takes a parameter. Functions called
during mutations must also return the type ``Result<T, Error>``.

The last function is the overridden destructor. This is not required but can be nice
if you need to clean up any connections to the subsystem between queries.

In the ``model.rs`` file there are also several other very simple structs which
don't have any functions implemented for them: ``SetPower``, ``ResetUptime``,
and ``CalibrateThermometer``. These are used as wrappers around scalar values
returned by various mutations in ``schema.rs``.

schema.rs
~~~~~~~~~

Now we will take a look inside of ``schema.rs``.  This file contains the query
and mutation models used by `Juniper <https://graphql-rust.github.io/juniper/current/>`__ to create
our GraphQL endpoints.

Queries
^^^^^^^

Queries allow us to fetch data from the subsystem. There is only one base ``Query``
struct needed in the ``schema.rs`` file.

.. code-block:: rust

    pub struct QueryRoot;

    /// Base GraphQL query model
    graphql_object!(QueryRoot : Context as "Query" |&self| {
        field subsystem(&executor) -> FieldResult<&Subsystem>
            as "Subsystem query"
        {
            Ok(executor.context().get_subsystem())
        }
    });


Inside of the `graphql_object macro <https://graphql-rust.github.io/juniper/current/types/objects/complex_fields.html>`__
we define each top-level query field. In this case there is just the one ``subsystem`` field.
In order to allow GraphQL access to the member functions (or variables) of the ``Subsystem``
struct we also apply the ``graphql_object`` macro to it:

.. code-block:: rust

    /// GraphQL model for Subsystem
    graphql_object!(Subsystem: Context as "Subsystem" |&self| {
        description: "Handler subsystem"

        field power() -> FieldResult<bool> as "Power state of subsystem" {
            Ok(self.power()?)
        }

        field uptime() -> FieldResult<i32> as "Uptime of subsystem" {
            Ok(self.uptime()?)
        }

        field temperature() -> FieldResult<i32> as "Temperature of subsystem" {
            Ok(self.temperature()?)
        }
    });

Here we create GraphQL field wrappers around each member of the ``Subsystem``
struct that we want exposed. The syntax ``Ok(self.func()?)`` allows the
translation of return type ``Result<T, Error>`` into ``FieldResult<T>``.

Mutations
^^^^^^^^^

Mutations allow us to call functions on the subsystem which cause change or
perform some action. Like the ``QueryRoot`` struct, we will only need one
top-level ``MutationRoot`` struct:

.. code-block:: rust

    pub struct MutationRoot;

    /// Base GraphQL mutation model
    graphql_object!(MutationRoot : Context as "Mutation" |&self| {

        // Each field represents functionality available
        // through the GraphQL mutations
        field set_power(&executor, power : bool) -> FieldResult<SetPower>
            as "Set subsystem power state"
        {
            Ok(executor.context().get_subsystem().set_power(power)?)
        }

    });


Each top-level mutation is exposed as an individual field. For each mutation
field there is a custom struct wrapping up the return values for that function.
Each of these structs must also have the graphql_object macro applied to them.

.. code-block:: rust

    /// GraphQL model for SetPower return
    graphql_object!(SetPower: Context as "SetPower" |&self| {
        description: "Enable Power Return"

        field power() -> FieldResult<bool> as "Power state of subsystem" {
            Ok(self.power)
        }
    });

These structs define fields which can then be used in the mutation to specify
which return data is desired.

Building and Running
~~~~~~~~~~~~~~~~~~~~

From inside of a Kubos SDK Vagrant box, navigate to the ``service`` folder of your
copy of the Rust service example.

Issue ``cargo build`` in order to build the service.

.. note::

    The ``cargo build`` command can be used to build any Rust service
    or crate from within the Vagrant box.

In order to run the service locally:

    - Verify that port 8123 is being forwarded out of your Vagrant box
    - Issue ``cargo run``

Once it is up and running you can use the GraphiQL endpoint to issue queries or mutations against
the service.

On your host machine, open a web browser and navigate to `http://127.0.0.1:8123/graphiql`.

Note: If your service is using something other than the default IP address and/or port, you'll need
to update the address in the URL.
