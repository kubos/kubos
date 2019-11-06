Kubos Mission Development
=========================

This section of the documentation walks through the necessary steps to get a satellite running KubOS from project initiation to flight ready.
This *only* covers the software development and preparation of the system as it pertains to KubOS, and does not cover hardware testing, power/thermal design and profiling, or any other mission development aspects that aren't software.
The following steps don't have to be done precisely in this order, but this is generally expected to be the optimal order.

1. Hardware Integrations
------------------------

Although there is already quite a bit of :doc:`supported hardware <../index>` in KubOS, your mission might have some hardware that is not yet supported, or unique to your mission (such as a payload).
:doc:`Hardware services <../ecosystem/services/hardware-services>` must be developed for any hardware that is not already supported.

Radio Integrations
__________________

Radios have an additional step that's required to finish integrating them into the system.
The hardware service only exposes the commands and telemetry for the radio hardware, but does not expose the uplink or downlink interface(s).
To expose their downlink and uplink capability to the rest of the system, we provide a :doc:`communication service framework. <../ecosystem/services/comms-framework>`

2. Payload Integration(s)
-------------------------

Besides avionics hardware, you likely have a payload you want to integrate for your mission.
KubOS provides a payload service guide for integrating payloads into KubOS in the *recommended* way:

- :doc:`Payload Services <../ecosystem/services/payload-services>`

We recommend all payload services follow this guide in order to keep system component interoperability simple and organized.
However, there is no hard requirement forcing payloads to conform to our specifications.
We want users to retain complete control over their systems and understand there may be cases where following our architecture isn't the best option.
For example, there might be a payload device which requires a lower latency than our communication flow provides.
In this case, it would be better for the appropriate mission application to be able to directly communicate with the device, rather than via a payload service.

3. Mission Applications
-----------------------

In KubOS, services are used primarily to expose functionality of underlying hardware.
They should be expected to only perform a minimal amount of decision-making (for example, kicking a watchdog at a pre-defined interval).
Instead, we rely on :ref:`mission applications <app-docs>` to handle the decision making for the mission.
Kubos has some mission applications that are open sourced in the repo, and others that we can offer to aid in mission development.
We've listed the typical necessary applications:

- :doc:`Mission Needs <mission-needs>`

.. toctree::
    :hidden:

    mission-needs
    deployment

4. Scheduling Tasks
-------------------

Applications in KubOS are generally designed to be single purpose and run relatively quickly.
The application itself should not be concerned with when or how often it is run. That is where
the scheduler comes in. We rely on the :ref:`scheduler service <scheduler-service>` to schedule
application execution at designated times or at pre-defined intervals.

We recommend all missions develop a thorough safe mode to ship as the mission's default schedule
and to use in fail over situations. Each application area in the :doc:`mission needs <mission-needs>`
doc should be considered for inclusion in the default safe mode schedule.

5. Update and Recovery
----------------------

KubOS uses similar procedures for both updating and recovering the operating system.
We *highly* recommend familiarizing yourself with both procedures during development, well before launch.

Updating KubOS
______________

The process for updating the operating system can be reviewed here: :doc:`KubOS update process. <../ecosystem/linux-docs/kubos-linux-upgrade>`
The process for updating mission applications can be reviewed here: :doc:`application service guide. <../ecosystem/services/app-service>`
Both should be reviewed and tested on your hardware prior to launch.

Kubos offers `SLAs <https://www.kubos.com/kubos/>`__ to aid in this process.

Recovery in KubOS
_________________

The :doc:`KubOS recovery process <../ecosystem/linux-docs/kubos-linux-recovery>` is defaulted to be configured for a development environment, where you have access to the hardware and are actively developing on it.
As a result, it **requires** augmentation before being flight ready, as it is not initially configured for desired on-orbit behavior.
The augmentation will likely be limited to changing the alternate boot behavior in the event of multiple failed boots.

Before you finish development, we recommend studying the recovery process and augmenting it as necessary to cover any possible edge cases that your hardware or software might encounter.
In addition, we also recommend doing several test cycles forcing the recovery to take place under various conditions, verifying that your mission specific code is effectively integrated into the recovery process.

Kubos offers `SLAs <https://www.kubos.com/kubos/>`__ to aid in the augmentation of the process and/or auditing your mission's recovery process.

6. Flight Readiness
-------------------

You have all the hardware integrated, you have all the required mission applications, and you've tested everything to your heart's content...you're ready for launch.
We've created a checklist for the likely steps to the prepare the software to be launch-ready:

- :doc:`Flight Ready <flight-ready>`

Please note that this is only a recommended list of steps, and will definitely change depending on your launch provider, mission requirements, and hardware configuration.

.. toctree::
    :hidden:

    flight-ready
