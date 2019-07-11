Typical Required Mission Applications
=====================================

Every mission is unique, but with this doc, we've attempted to highlight what applications your mission will most likely need.

Telemetry Collection
--------------------

Since all :ref:`services are passive <service-docs>`, it falls to an application to collect and store telemetry in the telemetry database.
Generally, this should poll all hardware services on a regular basis and pull all available telemetry.
A one-minute polling cycle is generally sufficient for most telemetry items.

.. TODO: merge the example app and update to say this: "Kubos provides an `example of this application, <#####>`__ and augmenting it for your mission should be simple given that all hardware services follow the service outline."

Housekeeping
------------

For each critical piece of avionics hardware or critical mission aspect, there should be an accompanying housekeeping mission application.
It is essential to separate these out as much as possible to reduce the complexity of a given application,
and to reduce possibility of global failure due to an edge case in a single app.
Some typical housekeeping applications you will most likely include are:

- OBC
- ADCS/GPS
- Power

These are by no means the *only* housekeeping applications that should be included, but would take care of most of the housekeeping for a given mission.

OBC
~~~

The OBC housekeeping application should ensure the OBC itself and the critical processes are running smoothly.
Some suggested tasks for this application would be:

- Clean the telemetry database
- Check file system and memory usage
- Issue a test query against services

Kubos provides an `example of this application <https://github.com/kubos/kubos/blob/master/apps/obc-hs/README.rst>`__,
which executes all of the above tasks on an hourly interval.

ADCS/GPS
~~~~~~~~

The ADCS/GPS housekeeping app will ensure the hardware itself is behaving normally, checking critical telemetry items to ensure nothing is outside the bounds of normal operation, taking action as appropriate.
Example action would be regularly updating the ADCS system with the latest time, position, and velocity data from the GPS.

Kubos offers housekeeping apps for its supported ADCS and GPS systems.
You can reach out to us on `Slack <https://slack.kubos.co>`__ or through the `website <https://www.kubos.com/kubos/>`__ for more information about these.

Power
~~~~~

The power management housekeeping application should monitor the Battery and EPS systems, taking critical autonomous recovery action where appropriate.
Some examples of triggers and actions:

- Shutting off non-essential hardware when battery reaches critically low status
- No charging detected for X time period
- Cancelling operations and going into power generation state
- Battery temperature monitoring

Kubos offers housekeeping apps for its supported power systems.
You can reach out to us on `Slack <https://slack.kubos.co>`__ or through the `website <https://www.kubos.com/kubos/>`__ for more information about these.

Deployment
----------

The deployment application should handle the required sequence during the initial deployment from the launch vehicle.
We've provided a guide to the recommended behavior of this application:

- :doc:`deployment`

Kubos offers configurable deployment applications for customer missions, as well as SLAs for helping develop and/or reviewing your mission's deployment application.
You can reach out to us on `Slack <https://slack.kubos.co>`__ or through the `website <https://www.kubos.com/kubos/>`__ for more information about these.

Beacon
------

The beacon application should regularly downlink a subset of critical telemetry data via the communication service.
This telemetry should be the absolute minimum required to assess overall system health.

Kubos offers configurable beacon applications for customer missions.
You can reach out to us on `Slack <https://slack.kubos.co>`__ or through the `website <https://www.kubos.com/kubos/>`__ for more information about these.

Operations
----------

All of the above applications are scoped to the core functionality of a spacecraft.
They ensure that the spacecraft is functioning properly within operational bounds.
But, they don't actually *complete* your mission.
Operations applications should deliver the functionality that is the objective of the mission itself.
For example:

- Command the ADCS to the appropriate attitude and take a photo of a commanded location
- Collect, store, and downlink specialized sensor data
- Process onboard sensor data to generate actionable beacons

An app can really do anything you want it to, but we suggest you keep them as simple as possible to reduce complexity.
If you find yourself building several modes into an operational application, maybe split it into several smaller ones that are each dedicated for the specific mode.

If you want help architecting or developing your operations applications, Kubos offers `SLAs <https://www.kubos.com/kubos/>`__ to aid in mission development.
