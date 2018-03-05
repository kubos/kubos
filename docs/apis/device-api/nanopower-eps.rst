Using the NanoPower API in a Kubos Project
==========================================

This document covers the particular capabilities and requirements of the Kubos API for the 
`GOMspace NanoPower P31u <https://gomspace.com/Shop/subsystems/power-supplies/nanopower-p31u.aspx>`__.

Reference Documents
-------------------

GOMspace
~~~~~~~~

    - `Datasheet <to-do link>`__
    - `Manual <to-do manual>`__

Kubos
~~~~~

    - `Kubos Documentation <http://docs.kubos.co/latest/index.html>`__
    - `Creating a Kubos Project <http://docs.kubos.co/latest/linux-docs/first-linux-project.html>`__
    - `Configuring a Kubos Project <http://docs.kubos.co/sphinx/sdk-docs/sdk-project-config.html>`__
    - `Using Kubos Linux <http://docs.kubos.co/sphinx/linux-docs/using-kubos-linux.html>`__
    
Initialization
--------------

:cpp:func:`k_eps_init`

Run-Time Configuration
----------------------

The ISIS antenna system supports in-flight configuration changes via the :cpp:func:`k_eps_configure` function.
The function takes a :cpp:type:`KANTSController` value which indicates which microcontroller should be used
to issue future antenna system commands.

System Config
~~~~~~~~~~~~~

k_eps_configure_system
k_eps_reset_system_config
k_eps_get_system_config

:cpp:type:`eps_system_config_t`

Battery Config
~~~~~~~~~~~~~~

k_eps_configure_battery
k_eps_save_battery_config
k_eps_reset_battery_config
k_eps_get_battery_config

:cpp:type:`eps_battery_config_t`

Controlling Outputs
-------------------

k_eps_set_output
k_eps_set_single_output

Controlling Inputs
------------------

k_eps_set_input_mode
k_eps_set_input_value

Controlling Heaters
-------------------

k_eps_set_heater
  
Watchdog
--------

Each antenna microcontroller has a watchdog which will restart the system if it has not been fed within the required interval.

.. warning::

    TODO: watchdog EEPROM lifetime warning. Timeout in seconds, but should add up to HOURS

There are three provided functions to assist with maintenance:

    - :cpp:func:`k_eps_watchdog_kick` - Send a single kick command to each microcontroller's watchdog
    - :cpp:func:`k_eps_watchdog_start` - Start a thread to send a kick command every `n` seconds to keep the watchdogs from starving
    - :cpp:func:`k_eps_watchdog_stop` - Terminate the watchdog thread
    
Other Functions
---------------

    - :cpp:func:`k_eps_reset` - Hard reset the EPS
    - :cpp:func:`k_eps_reboot` - Soft reset the EPS (output power will not be affected)
    - :cpp:func:`k_eps_reset_counters` - Reset the system counters (boot count, watchdog reboot counts, etc)
    - :cpp:func:`k_eps_get_housekeeping` - Get current housekeeping information
    - :cpp:func:`k_eps_passthrough` - Pass a command packet directly through to the device
    
Telemetry Information Available
-------------------------------

- ``vboost[3]``
- ``vbatt``
- ``curin[3]``
- ``cursun``
- ``cursys``
- ``curout[6]``
- ``output[8]``
- ``output_on_delta[8]``
- ``output_off_delta[8]``
- ``latchup[6]``
- ``wdt_i2c_time_left``
- ``wdt_gnd_time_left``
- ``wdt_csp_pings_left[2]``
- ``counter_wdt_i2c``
- ``counter_wdt_gnd``
- ``counter_wdt_csp[2]``
- ``counter_boot``
- ``temp[6]``
- ``boot_cause``
- ``batt_mode``
- ``batt_maxvoltage``
- ``batt_safevoltage``
- ``batt_criticalvoltage``
- ``batt_normalvoltage``
- ``ppt_mode``
- ``battheater_mode``
- ``battheater_low``
- ``battheater_high``
- ``output_normal_value[8]``
- ``output_safe_value[8]``
- ``output_initial_on_delay[8]``
- ``output_initial_off_delay[8]``