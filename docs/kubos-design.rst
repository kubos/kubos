KubOS Design
============

This document gives an introduction to the philosophy behind KubOS, and how that philosophy is reflected in the design of KubOS.
If you're looking to dive into things immediately, check out our :doc:`Getting Started <getting-started/index>` section!
If you want a description of each of the components in KubOS and their capability, check out the :doc:`KubOS Ecosystem <ecosystem/index>` document, and the numerous pages it links to.

What is KubOS?
--------------

KubOS is a collection of microservices that accomplish critical functionality required of flight software (FSW),
run within a highly fault tolerant and recoverable operating system,
and provide a safe and effective development environment for mission specific FSW applications.

Microservices
-------------

The microservice architecture of the critical FSW functionality keeps our system remarkably stable,
while maintaining agility for feature and technology improvements.
The diagram below shows the typical architecture for a mission running KubOS:

.. figure:: images/mission_diagram.png
    :align: center
    :alt: block diagram depicting the software of a typical kubos-powered mission

    The software powering a typical kubos-based mission with components provided by KubOS in blue and components that need to be implemented for the specific mission payload in red.

Each of the boxes are independent processes, or *microservices*.
For example, we can revise the communication service to support a new radio, and the binaries of *all other services* are unchanged.
This can be useful between missions, to truly maintain flight heritage on the components that need no revisions,
or it can be useful during a mission, as each process can be updated, on orbit, reliably and independently.

Operating System
----------------

KubOS uses a combination of Linux and U-boot to make up its operating system.

Linux is a far more abstract OS than is typically used for satellite software,
which does mean it is more resource intensive.
But, as satellite on-board computers have become substantially more capable,
some (of the many) benefits of Linux start to become more important:

- Services are completely portable between any KubOS-supported OBC, requiring no code changes.
- Tooling is available to easily control process boundaries and resource allocation.
- Massive ecosystem of software, tooling, and experienced developers.
- Mature software already running on billions of systems.

Since Linux is far more complex, it does come with inherent risks, which KubOS combats by pairing it with U-boot.
U-boot is a widely-used bootloader that manages the Linux kernel and the core of our system,
capable of automatically failing over to stored backups should a catastrophic event occur.
This bootloader also gives us the capability to update the entire operating system *during flight*, if a single process update is insufficient.

.. _real-time:

Real-Time and KubOS
^^^^^^^^^^^^^^^^^^^

Linux, inherently, is not a real-time operating system (RTOS), and there are often strict timing requirements for spacecraft missions.
Why then did we still pursue Linux rather than an RTOS environment?
How do we expect to tackle these requirements?

How to go about addressing real-time requirements on a spacecraft running KubOS depends on the actual requirement itself.
A very brief summary of what a *hard* real-time requirement is:

- I need **X** thing to happen within **Y** amount of time after receiving **Z** signal.
- I need to *guarantee* the response time will *always* be less than **Y**.

A hard real-time requirement is where both of these must be satisfied.

Do you need *hard* real-time performance?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Not all satellites have hard real-time requirements, but rather they have general timing expectations.
If you want a response within tens of milliseconds, this is already accomplished with normal Linux usage.
Limiting code complexity can help further reduce this latency.

You need hard real-time. What's possible in Linux?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Real-time requirements are very possible to achieve directly within a Linux environment.
In fact, millions of servers around the world with strict real-time requirements are currently running Linux.
But there are limits to what is achievable for hard real-time within a Linux environment.
Worst-case response time guarantees vary greatly depending on how much you've optimized Linux,
but can be as low as **30 microseconds**.
This is likely enough to satisfy most hard real-time constraints.
`Come talk to us <https://slack.kubos.co/>`__ about how to achieve your hard real-time requirement in KubOS!

Still not enough. What can you do?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

In the event your mission needs a lower latency than Linux can achieve,
or you don't want to introduce the complexity associated with the real-time optimization,
it's possible to pair your primary OBC with an FPGA or microprocessor.
The dedicated device will satisfy the real-time functionality for your specific task.
This is already frequently implemented in microsat and nanosat missions with standalone ADCS modules containing commandable microprocessors running ADCS algorithms.
There are several benefits for adopting this architecture:

- Developing the non-real-time components of your mission software in a much easier development environment.
- Leveraging existing KubOS (and Linux) tooling and functionality for the rest of your mission software.
- Limiting the risk and severity of impact this highly complex task has on the rest of your system.
- Verifying the hard real-time performance is much simpler, as it has substantially less functionality to test.

Developing in KubOS
-------------------

Writing flight software is not an easy task.
The satellite software industry has arrived at the point where developer hours and development timelines are becoming more of a constraint than OBC capability.
As a result, KubOS strives to enable developers to *quickly* produce *reliable* mission software, as *efficiently* as possible.

To uphold these core priorities, KubOS focuses on enabling mission developers to write small, standalone applications that leverage the microservices and operating system we provide.
This allows applications to be understood, revised, updated, and tested much more easily, as the total "code change" per mission ends up being only these small applications.

There is obviously much, much more to what applications and services can do and how they interact, and we suggest diving into the :doc:`KubOS Ecosystem <ecosystem/index>` documentation for more information.
Or, now that you understand what we've set out to do and why, you can :doc:`get started developing <getting-started/index>`!
