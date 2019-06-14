Preparing for Flight Readiness
==============================

This checklist is a **framework** for when you are preparing KubOS to integrate your spacecraft into the launch vehicle.
This is *only* valid for the software, although it might reference hardware to give context.
Also, it is important to add steps to account for your mission specific apps and services (or whatever else you add to the system).

Prepare Linux
-------------

1. Build the flight linux image, including any core and/or hardware services you need for your mission.
2. Load the flight image onto the spacecraft.
3. Build the recovery and upgrade rootfs images.
4. Load them into the recovery and upgrade partitions.

Prepare Services
----------------

1. Boot up the bus.
2. Ensure any included hardware services are properly communicating with hardware by issuing a ``noop`` mutation to each.
3. Ensure any payload services you're including are properly communicating with their respective payload (hopefully by running the ``noop`` mutation you dutifully included).
4. Ensure your ``config.toml`` is appropriate for your bus configuration.
5. Configure your :doc:`logging and log retention <../os-docs/linux-docs/logging>` for your system memory requirements.
  a. We recommend that you limit logging to ``info`` and higher if you are using a file location that is prone to wearing out, as ``debug`` logs can cause significant amounts of logs to be generated.


Prepare Applications
--------------------

1. Register stable versions of all applications (if you include multiple versions, make sure the correct one is active)
2. Reboot the bus, and ensure all apps that should start on boot are properly started.
3. If you followed the :doc:`Deployment doc <deployment>` for your deployment app, or you are using the KubOS deployment app, you will need to:
  a. Ensure the ``deployed`` U-boot environment variable is set to ``False``.
  b. Ensure the ``remove_before_flight`` U-boot environment variable is set to ``False``.
  c. Ensure the ``deploy_start`` U-boot environment variable is unset.
