************
Rust Service
************

This is a quick overview of the payload service written in Rust.

Libraries
=========

This payload service and future rust-based services will be written using
the following external crates:

- `Juniper <https://github.com/graphql-rust/juniper>`_ - GraphQL server library

- `Iron <http://ironframework.io/>`_ - HTTP library


The ``Cargo.toml`` in the example payload service gives a good list of crate
dependencies to start with.


Example Source
==============

The example rust service is found in the
`examples <https://github.com/kubos/kubos/tree/master/examples>`_ folder in the
`kubos/kubos <https://github.com/kubos/kubos>`_ repo. There is a `rust-service <https://github.com/kubos/kubos/tree/master/examples/rust-service>`_
folder which contains two folders:

``extern-lib`` - This is an example rust crate showing how to link in external C source.

``service`` - This crate contains the actual rust service.

The contents of the ``service`` folder:

``Cargo.lock`` - Cargo `lock <https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html>`_ file

``Cargo.toml`` - Cargo `manifest <https://doc.rust-lang.org/cargo/reference/manifest.html>`_ file

``src`` - Contains the actual Rust source.

The contents of the ``service/src`` folder:

``main.rs`` - Contains the main/setup function of the service. May need minor customization but not much.

``model.rs`` - Describes the hardware model exposed to GraphQL and contains calls down to lowel-level APIs.

``schema.rs`` - Contains the actual GraphQL schema models used to generate the GraphQL endpoint.

We will now take a closer look at ``model.rs`` and ``schema.rs`` and break down
the pieces required to expose hardware APIs through the service.

model.rs
========

The ``model.rs`` file contains structures and functions used to wrap low-level device APIs
and provide abstractions for the GraphQL schema to call into. Looking inside of the ``model.rs``
file you will see several ``struct`` declarations. We'll start with the ``Subsystem``:

.. code-block:: rust

  ``pub struct Subsystem;``

Here we have a struct which is used to model a Subsystem. In this example the struct
is given no member variables for persistence, all data is obtained through function
calls for real-time results.

Here is an abbreviated set of functions implemented for the ``Subsystem`` struct:

.. code-block:: rust

	impl Subsystem {
	    /// Creates new Subsystem structure instance
	    /// Code initializing subsystems communications
	    /// would likely be placed here
	    pub fn new() -> Subsystem {
		println!("getting new subsystem data");
		// Here we call into an external C based function
		extern_lib::k_init_device();
		Subsystem {}
	    }

	    /// Power status getter
	    /// Code querying for new power value
	    /// could be placed here
	    pub fn power(&self) -> Result<bool, Error> {
		println!("getting power");
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
		extern_lib::k_terminate_device();
	    }
	}

The ``new`` function is the ``Subsystem`` constructor. It can be used to establish
a connection with the hardware if neccesary. This function is called once per
query or mutation and produces the struct instance used.

The ``power`` function is an example of a function called during a query. These
functions called by GraphQL functions must return the type ``Result<T, Error>``
in order to properly unpack valid data vs an error message.

The ``set_power`` function is an example of a function called during a mutation.
It is essentially the same as ``power`` but takes a parameter. Functions called
during mutations must also return the type ``Result<T, Error>``.

The last function is the overridden dustructor. This is not required but can be nice
if you need to clean up any connections to the subsystem between queries.

In the ``model.rs`` file there are also several other very simple structs which
don't have any functions implemented for them: ``SetPower``, ``ResetUptime``,
and ``CalibrateThermometer``. These are used as wrappers around scalar values
returned by various mutations in ``schema.rs``.

schema.rs
=========

Now we will take a look inside of ``schema.rs``.  This file contains the query
and mutation models used by `juniper <http://juniper.graphql.rs/>`_ to create
our GraphQL endpoints.

Queries
-------

Queries allow us to fetch data from the subsystem. There is only one base ``Query``
struct needed in the ``schema.rs`` file.

.. code-block:: rust

    pub struct QueryRoot;

    /// Base GraphQL query model
    graphql_object!(QueryRoot : Context as "Query" |&self| {
        field subsystem(&executor) -> FieldResult<&Subsystem>
            as "Subsystem query"
        {
            // I don't know if we'll ever return anything other
            // than Ok here, as we are just returning back essentially
            // a static struct with interesting function fields
            Ok(executor.context().get_subsystem())
        }
    });


Inside of the `graphql_object macro <http://juniper.graphql.rs/types/objects/complex_fields.html>`_
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
---------

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
====================

The payload service provided in the ``examples`` folder can be compiled by running
this command ``cargo kubos -c build`` from inside of the Kubos SDK Vagrant box.

The service can then be run by this command ``cargo kubos -c run``. You will want
to check that port 5000 is forwarded out of your Vagrant box before testing the service.
Once it is up and running you can navigate to http://127.0.0.1:5000/graphiql for
the interactive GraphiQL interface.
