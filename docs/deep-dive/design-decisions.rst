KubOS Design Decisions
======================

Buildroot
---------

When deciding on which tool to use in order to configure and build Kubos Linux, we compared
`Buildroot <https://buildroot.org/>`__ and `Yocto <https://www.yoctoproject.org/>`__.

Both systems are widely adopted, actively maintained, and have an easy learning curve.

We decided on Buildroot for a few reasons:

- Buildroot tends towards smaller file sizes. Configuration options are only on by default if they
  are absolutely needed
- It uses Makefiles and Kconfig files, which engineers are more likely to already be familiar with
- At the time, we felt that Buildroot had better user documentation available

GraphQL
-------

It's light-weight

Rust
----

It's statically typed and fast.

(why not ada?)

Python
------

We anticipate that many of the people who will be developing a mission won't have a programming
background.

Python is one of the most common beginning programming languages.
It has an easy learning curve and has a plethora of tutorials available.

CCSDS
-----

Microservices
-------------

Having services provide the interfaces to the underlying hardware allows us to very tightly control
what is allowed to access the hardware, when it's accessed, and how it's accessed.

Services guarantee:

- That the hardware will always be accessed with the correct protocol and valid commands
- Only one entity will communicate with the hardware at a time

Similar reasoning applies to the telemetry database service.
Because we're using SQLite, there must only ever be one entity attempting to write to the database
at a time.
The telemetry database service will always be that one entity.

SQLite
------

It's small

Busybox
-------

It's small

Custom File Transfer
--------------------

Need to handle dropped packets and super asynch communication