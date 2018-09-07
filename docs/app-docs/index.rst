Kubos Mission Applications
==========================

Mission applications include anything that makes decisions or governs autonomy on the satellite, as well as any other applications designed to address mission-specific concerns.

Applications can be used to enable the satellite to act autonomously. Some basic versions of this are:

- Querying hardware services and storing telemetry
- Creating and broadcasting telemetry beacons that change depending on satellite state
- Controlling deployment procedures

Applications are also used to accomplish specific mission goals. Some possible examples are:

- Image analysis
- Payload operation
- Payload monitoring
- `Deployment <deployment>`

KubOS is built to support multiple mission application files existing and running on a system concurrently.
This allows customers to break their logic into specific, siloed portions which can be updated or replaced piecemeal
as issues arise or as new features are added.

Additionally, :doc:`mission applications <app-guide>` can be easily configured to execute different logic depending on when
and how they are run. For example, when run on-demand, versus when run at startup.

Mission applications are monitored and managed by the :doc:`Kubos applications service <app-service>`.

.. todo::

    This service is responsible for starting all applications at the requested time (or times), rebooting applications if they crash unexpectedly,
    and even rolling back applications to a previous version if errors persist. (except not right now, because it's not coded yet. Update this with the reality before publishing)

    TODO: Diagram of service/app/user interactions?

.. toctree::
    :caption: Applications Docs
    :hidden:

    Applications Development Guide <app-guide>
    Kubos Applications Service Guide <app-service>
    Deployment Application Guide <deployment>
