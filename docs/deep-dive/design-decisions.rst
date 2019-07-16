KubOS Design Decisions
======================

This doc records the high-level reasoning for some of our major design decisions made while
architecting KubOS.

Linux vs RTOS
-------------

Check out the :ref:`real-time section <real-time>` in our KubOS Design doc for information about this decision.

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

We wanted a command format that was human readable, to make integration and testing easier.
We analyzed a few different types, but settled on GraphQL because it also gave us a communication
layer and library support that other human readable command formats didn't.

We also appreciated having the ability to cherry-pick the data points which should be returned in a
particular request.
This methodology shrinks the amount of bandwidth required to send and receive desired data.

Additionally, you can request multiple different data points to be returned in a single request
(unlike with REST), reducing the overall number of HTTP messages which need to be sent.

Rust
----

Rust is a statically-typed language, so it is able to catch many common mistakes at compile time.
The sooner you can catch a mistake, the sooner you can fix it, and the less likely it is to cause
other problems with your system.
Additionally, memory safety, the root of some of the most difficult-to-solve software issues, is a
first-class citizen.

With these protections, Rust still maintains a C-like flexibility and speed.
This allows us to be confident that we can implement any feature we need in the future.

Python
------

We anticipate that many of the people who will be developing a mission won't have a programming
background.
While we use Rust internally, we acknowledge that it is more suitable for experienced programmers.

Python is one of the most common beginning programming languages.
It has an easy learning curve and has a plethora of tutorials available.

Additionally, Python is already widely used within the aerospace community.

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

We use a SQLite for our telemetry storage.
We decided SQLite was the best choice because it's light-weight, yet fully-featured, and has
extensive heritage within the embedded device world.

Busybox
-------

Busybox provides an alternative to most of the common Linux command utilities.
We chose to use it because it allows us to reduce the amount of space required for the root file
system.
