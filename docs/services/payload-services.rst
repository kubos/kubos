****************
Payload Services
****************

Payload services are essentially hardware services but custom designed
for mission payload hardware. They share the same architecture as the hardware
services, exposing low-level device APIs through a GraphQL interface.

The Kubos SDK includes an example payload service written in both Python
and Rust. Here we will discuss the example Python-based payload service, its
different pieces and how to construct a custom payload service.

The current guide for working with Python within the Kubos SDK can be
found :doc:`here <../sdk-docs/sdk-python>`.

Python Service
==============

The two example payload services can be found in the
`examples <https://github.com/kubos/kubos/tree/master/examples>`_ folder in the
`kubos/kubos <https://github.com/kubos/kubos>`_ repo. The python service is found
in the `python-service <https://github.com/kubos/kubos/tree/master/examples/python-handler>`_
folder. Inside of the `python-service` folder you will find several files and a folder:

``config.yml`` - This YAML file holds configuration options for the GraphQL/HTTP endpoint.

``README.rst`` - Description file for service.

``requirements.txt`` - Python module requirements file with list of module/version dependencies.

``service.py`` - Boilerplate service code which reads the config file and starts up the GraphQL/HTTP endpoint.

``service/`` - This folder holds the guts of the service's source.

The service folder contains the main files which will need modifying when building a custom payload service:

``__init__.py`` - This empty file belongs in the `service/` folder to give `service.py` access to the modules within.

``app.py`` - Another boilerplate service file. This one should not require any customization.

``models.py`` - Describes the hardware model exposed to GraphQL and contains calls down into lower level APIs.

``schema.py`` - Contains the actual GraphQL models which are used to generate the GraphQL endpoint.


We will now take a closer look at `models.py` and `schema.py` to see what exactly it takes to expose a hardware
API through the service.

models.py
=========

Inside of the example `models.py` file there are `Subsystem` and `Status` classes. Both of these classes must be subclasses of `graphql.ObjectType <http://docs.graphene-python.org/en/latest/types/objecttypes/>`_ from the `graphene <http://docs.graphene-python.org/en/latest/>`_ module.

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

Member variables can be added if any persistent data needs to be stored. Member functions are called by the GraphQL schema and are used to call into low level device API functions.

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
=========

Now lets look inside of `schema.py`. This file contains the models used by `graphene` to create our GraphQL endpoint.

Queries
-------

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

Any member variables of the type `graphene.Field` become top-level fields accessible by queries. Because we are using the `Subsystem` class, which is also a `graphene.ObjectType`, members of that class become accessible by queries. Each graphene field requires a resolver function named `resolve_fieldname` which returns back an object of the field's class type.  In this case we call `_subsystem.refresh()` to load the latest data into the global `_subsystem` object and return it.

The above class would enable the following query for subsystem power status:::

    {
        subsystem {
            powerOn
        }
    }

Mutations
---------

Mutations allow us to call functions on the subsystem which cause change or perform some action. Like the `Query` class we will only need one top level `Mutation` class.

.. code-block:: python

	class Mutation(graphene.ObjectType):
	    """
	    Creates mutation endpoints exposed by graphene.
	    """

	    power_on = PowerOn.Field()

Like with the `Query`, each `Field` member becomes a top-level mutation. However for mutations we will create a new class for each mutation field.

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

The `Arguments` class describe any argument fields needed for this mutation. The line ``Output = Status`` describes the class type this mutation should return. The ``mutate`` function performs the actual work of the mutation and must return back an object of the type specified in the ``Output`` line. The above classes enable the following mutation:::

    mutation {
        powerOn(power:false) {
            status
        }
    }

Running the example
===================

Getting the example service up and running is fairly simple. First you must make sure you have the necessary python dependencies installed. If you are using the Kubos SDK vagrant box then these will already be installed. Otherwise you will need to run ``pip install -r requirements.txt``.

Once the dependencies are in place you can run ``python service.py config.yml`` and the example service should begin. You will know that it is running if the command line output says ``* Running on http://0.0.0.1:5000/ (Press CTRL+C to quit)``. You can now point a web browser to http://127.0.0.1:5000/graphiql to access a `graphical GraphQL interface <https://github.com/graphql/graphiql>`_. Here you can run quries and mutations against the GraphQL endpoints and see the results.

.. note::

   If you are running the example from within the Vagrant box then you may need
   some additional configuration.

By default the Vagrant box does not forward any ports. In order to access the HTTP
interface of the service running inside of the Vagrant box we need to forward
the port it is using. To do so you will need to add the following line to
your ```Vagrantfile``` (after ``Vagrant.configure("2") do |config|``)::

  config.vm.network "forwarded_port", guest: 5000, host: 5000

Now restart the vagrant box with ``vagrant reload``. You should now have the ability
to run the python service inside the Vagrant box and access it from the outside
at http://127.0.0.1:5000.
