Kubos Mission Applications
==========================

TODO: Re-work for the new "Mission Development" umbrella.
It includes developing payload services and whatnot

Mission applications include anything that makes decisions or governs autonomy on the satellite, as well as any other applications designed to address mission-specific concerns.

Applications can be used to enable the satellite to act autonomously. Some basic versions of this are:

- Querying hardware services and storing telemetry
- Creating and broadcasting telemetry beacons that change depending on satellite state
- Controlling deployment procedures

Applications are also used to accomplish specific mission goals. Some possible examples are:

- Image analysis
- Payload operation
- Payload monitoring
- :doc:`Deployment <deployment>`

KubOS is built to support multiple mission application files existing and running on a system concurrently.
This allows customers to break their logic into specific, siloed portions which can be updated or replaced piecemeal
as issues arise or as new features are added.

Additionally, :doc:`mission applications <../os-docs/apps/app-guide>` can be easily configured to execute different logic depending on when
and how they are run. For example, when run on-demand, versus when run at startup.

Mission applications are monitored and managed by the :doc:`Kubos applications service <../os-docs/services/app-service>`.

TODO: Typical mission needs:

- apps (housekeeping, beacon, telemetry)
- services
- etc

.. toctree::
    :caption: Applications Docs
    :hidden:

    mission-needs
    mission-dev-guide
    Deployment Application Guide <deployment>
    flight-ready
