Kubos Mission Applications
==========================

Mission applications include anything that makes decisions or governs autonomy on the satellite, as well an any other applications designed to address mission-specific concerns. 

Applications can be used to enable the satellite to act autonomously. Some basic versions of this are:

- Querying hardware services and storing telemetry
- Creating and broadcasting telemetry beacons that change depending on satellite state
- Controlling deployment procedures

Applications are also used to accomplish specific mission goals. Some possible examples are:

- Image analysis 
- Payload operation 
- Payload monitoring

Mission applications are monitored and managed by the `Kubos applications service <app-service>`.
This service is responsible for starting all applications at the requested time (or times), rebooting applications if they crash unexpectedly,
and even rolling back applications to a previous version if errors persist. (except not right now, because it's not coded yet. Update this with the reality before publishing)

Diagram of service/app/user interactions

TODO: Add blurb somewhere in here about the app framework and the concept of run levels
    
.. toctree::
    :caption: Applications Docs
    :hidden:
    
    Kubos Applications Service Guide <app-service>
    Applications Development Guide <app-guide>