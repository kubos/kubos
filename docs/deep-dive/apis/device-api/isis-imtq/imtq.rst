Using the ISIS iMTQ in a Kubos Project
======================================

This document covers the particular capabilities and requirements of the Kubos API for the
`ISIS iMTQ magnetorquer <https://www.isispace.nl/product/isis-magnetorquer-board/>`__.
This doc focuses on the C API, however the Rust API interface is very similar.

The API is split into four distinct categories:

  - Core functions (connection init/terminate, watchdog)
  - Operational functions (reset iMTQ, set coils, start tests)
  - Data request functions (get data, get telemetry, get test results)
  - Configuration (get/set/reset system parameters)

.. toctree::
    :maxdepth: 1
    
    ISIS iMTQ API - C <imtq_api_c>
    ISIS iMTQ API - Rust <imtq_api_rust>
     

Reference Documents
-------------------

ISIS
~~~~

    - iMTQ User Manual - The main guide for the iMTQ
    - iMTQ Options Sheet - Allows customers to specify non-default options that their device should be manufactured with

Kubos
~~~~~

    - :doc:`Creating a Kubos Project <../../../../tutorials/first-obc-project>`
    - :doc:`Using Kubos Linux <../../../../ecosystem/linux-docs/using-kubos-linux>`
    - :doc:`Working with the iOBC <../../../../obc-docs/iobc/working-with-the-iobc>`

Project Configuration
---------------------

There are several options which may be specified when initializing an iMTQ connection.
These option values should match what was specified in your iMTQ options sheet.

- bus - The I2C bus the iMTQ is connected to
- addr - The I2C address of the iMTQ device
- timeout - The watchdog timeout value, in seconds

Command Responses
-----------------

Each command sent to the iMTQ will return a response beginning with an :cpp:type:`imtq_resp_header` structure.

This structure contains several useful pieces of information:

    - The command that the response corresponds to
    - A status byte containing

        - A flag indicating whether this response has been fetched before
        - Flags indicating the validity of each axis' measurement data
        - An error code

The iMTQ API will automatically verify that the command echoed in the response matches the command that was sent.
Additionally, it will extract and return the error code as a :cpp:type:`KADCSStatus` value.

For commands returning axis measurement data, users should check the presence of the :c:macro:`RESP_IVA_X`,
:c:macro:`RESP_IVA_Y`, and :c:macro:`RESP_IVA_Z` flags. These flags are documented in section 3.2.4 of the
*iMTQ User Manual* and indicate that the corresponding axis' data might be invalid.

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>
    
    KADCSStatus status = ADCS_OK;
    imtq_axis_data dipole = {0};
    uint16_t time = 10;

    dipole.x = 42;
    dipole.y = 63;
    dipole.z = -125;
    
    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize iMTQ connection\n");
        return;
    }

    /* Start actuation */
    status = k_imtq_start_actuation_dipole(dipole, time);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start actuation (dipole): %d\n", status);
        return status;
    }

    /* Give it some time to run */
    const struct timespec TRANSFER_DELAY = {
        .tv_sec = 0,
        .tv_nsec = 100000000
    };

    nanosleep(&TRANSFER_DELAY, NULL);

    /* Get the commanded acuation dipole values */
    status = k_imtq_get_dipole(&dipole);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get coil currents: %d\n", status);
        return status;
    }
    
    /* Check the data validity */
    if (dipole.hdr.status & RESP_IVA_X)
    {
        printf("X-axis data might be invalid\n");
    }
    if (dipole.hdr.status & RESP_IVA_Y)
    {
        printf("Y-axis data might be invalid\n");
    }
    if (dipole.hdr.status & RESP_IVA_Z)
    {
        printf("Z-axis data might be invalid\n");
    }
    
    /* Print the results */
    printf("Command actuation dipole - X: %d Y: %d Z: %d\n",
                dipole.data.x, dipole.data.y, dipole.data.z);
                
    k_adcs_terminate();
    
Run-Time Configuration
----------------------

.. warning::

    **These configuration changes will not persist through reboot, including one triggered by the watchdog**

The ISIS iMTQ supports in-flight configuration changes and queries via the ``k_acds_configure`` function.
The desired configuration should be passed to the function as a JSON structure (defined as ``JsonNode *``)
which has been built using the CCAN/JSON library in the Kubos repo.

The user may either convert a character buffer containing JSON using ``json_decode`` and/or create a JSON structure manually
using ``json_mkobject`` and ``json_append_member``.

The iMTQ's configuration parameters are documented in section 3.4 of the *iMTQ User Manual*.

For increased code readability, these codes have been included in the API with pre-defined names.

As documented in Table 3-8 of the *iMTQ User Manual*, the configuration parameter values are not all the same type.
The ``k_adcs_configure`` function will automatically convert the requested value to the correct type.

.. warning:: The JSON structure used for configuration must be deleted **by the user** after ``k_adcs_configure`` has been called

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>
    
    #define STRINGIFY(x)            STRINGIFY2(x)
    #define STRINGIFY2(x)           #x
    
    KADCSStatus status = ADCS_OK;
  
    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize iMTQ connection\n");
        return;
    }
    
    /* Create the JSON structure with our desired configuration options */
    JsonNode * config = json_mkobject();
    json_append_member(config, STRINGIFY(MTM_INTERNAL_MAP_X), json_mknumber(3));
    json_append_member(config, STRINGIFY(MTM_INTERNAL_MAP_Y), json_mknumber(1));
    json_append_member(config, STRINGIFY(MTM_INTERNAL_MAP_Z), json_mknumber(5));
    
    /* Configure the iMTQ */
    status = k_adcs_configure(config);
    
    /* Delete the JSON structure now that we're done with it */
    json_delete(config);
    
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to configure iMTQ\n");
    }

    k_adcs_terminate();

System Mode
-----------

The ``k_adcs_set_mode`` function can be used to put the iMTQ into either idle mode or detumble mode.

Idle
~~~~

Putting the iMTQ into idle mode will cause any ongoing actuation to be immediately cancelled.

Detumble
~~~~~~~~

If detumble mode is specified, the :cpp:type:`adcs_mode_param` argument should be used to specify the amount of time,
in seconds, the iMTQ should spend in detumble mode before returning to idle.

This value may be zero, indicating that the iMTQ should remain in detumble mode indefinitely (or until changed by
another function call).

Self-Tests
----------

As documented in section 2.8 of the *iMTQ User Manual*, the iMTQ is capable of performing single-axis or all-axes
self-tests to verify that the system components are working correctly.

These tests can be run using the :cpp:func:`k_adcs_run_test` function.
This corresponds with commands TC-OP-08 and TC-DR-07 documented in the *iMTQ User Manual*.
The function takes an :cpp:type:`imtq_test_axis` value indicating which axis to test and a JSON parent object
(abstracted with :cpp:type:`adcs_test_results`) to which the results should be attached.

The test results are documented in the TC-DR-07 command in section 3.3 of the *iMTQ User Manual*

.. warning:: The JSON structure used for the self-test results must be deleted **by the user** once it is no longer of use.

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>

    KADCSStatus status;

    /* Make parent object */
    adcs_test_results test = json_mkobject();

    /* Get the data */
    status = k_adcs_run_test(TEST_X_POS, test);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ telemetry\n");
        json_delete(test);
        return ADCS_ERROR;
    }

    /* Print the results */
    char * temp = json_stringify(test, " ");
    puts(temp);
    free(temp);

    /* Free the memory */
    json_delete(test);

The printed results generated by the example might look like this::

    {
     "tr_init_error": 0,
     "tr_init_mtm_raw_x": -1937,
     "tr_init_mtm_raw_y": -2223,
     "tr_init_mtm_raw_z": 3145,
     "tr_init_mtm_calib_x": -14528,
     "tr_init_mtm_calib_y": -16688,
     "tr_init_mtm_calib_z": 23603,
     "tr_init_coil_current_x": -4,
     "tr_init_coil_current_y": -5,
     "tr_init_coil_current_z": -27,
     "tr_init_coil_temp_x": 24,
     "tr_init_coil_temp_y": 24,
     "tr_init_coil_temp_z": 24,
     "tr_posx_error": 0,
     "tr_posx_mtm_raw_x": -50935,
     "tr_posx_mtm_raw_y": -85279,
     "tr_posx_mtm_raw_z": 12577,
     "tr_posx_mtm_calib_x": -382013,
     "tr_posx_mtm_calib_y": -639608,
     "tr_posx_mtm_calib_z": 94343,
     "tr_posx_coil_current_x": 424,
     "tr_posx_coil_current_y": 2,
     "tr_posx_coil_current_z": 2,
     "tr_posx_coil_temp_x": 24,
     "tr_posx_coil_temp_y": 24,
     "tr_posx_coil_temp_z": 24,
     "tr_fina_error": 0,
     "tr_fina_mtm_raw_x": -1789,
     "tr_fina_mtm_raw_y": -2391,
     "tr_fina_mtm_raw_z": 3477,
     "tr_fina_mtm_calib_x": -13418,
     "tr_fina_mtm_calib_y": -17948,
     "tr_fina_mtm_calib_z": 26093,
     "tr_fina_coil_current_x": 1,
     "tr_fina_coil_current_y": -5,
     "tr_fina_coil_current_z": -16,
     "tr_fina_coil_temp_x": 24,
     "tr_fina_coil_temp_y": 24,
     "tr_fina_coil_temp_z": 24
    }

Telemetry
---------

The ``k_adcs_get_telemetry`` function is capable of returning two different categories of telemetry: nominal and debug.

Each function call will also return the current system status, as generated by :cpp:func:`k_imtq_get_system_state`.

.. warning:: The JSON structure used to get the telemetry must be deleted **by the user** once it is no longer of use.

This function takes two parameters:

    - The type of telemetry to return, specified by :cpp:type:`ADCSTelemType`
    - A pointer to a JSON structure (defined as ``JsonNode *``) to which the telemetry results should be added

.. warning:: The JSON structure used for the telemetry information must be deleted **by the user** once it is no longer of use. 

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>
    
    KADCSStatus status = ADCS_OK;

    /* Make parent object */
    JsonNode * telem = json_mkobject();

    /* Get the data */
    status = k_adcs_get_telemetry(NOMINAL, telem);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Error/s occurred while getting ADCS telemetry\n");
    }

    /* Print results */
    char * temp = json_stringify(telem, " ");
    puts(temp);
    free(temp);

    /* Free the memory */
    json_delete(telem);

Nominal
~~~~~~~

The nominal telemetry will contain the following information:

  - Housekeeping - All values returned as both raw ADCS and calculated engineering values.
    Corresponds with the information returned by TC-DR-09 and TC-DR-10.

      - Voltage of the digital supply
      - Voltage of the analog supply
      - Current of the digital supply
      - Current of the analog supply
      - Coil currents
      - Coil temperatures
      - MCU temperature

  - Data during last detumble iteration. Corresponds with the information returned by TC-DR-08.

      - Calibrated magnetometer data
      - Filtered magnetormeter data
      - B-Dot values
      - Commanded actuation dipole values
      - Command current values
      - Coil currents

  - Current magnetometer measurements - *Note: this information will only be returned if the iMTQ is in IDLE mode.*
    Corresponds with the operational commands TC-DR-02 and TC-DR-03 and information returned by TC-DR-02 and TC-DR-03.

      - Coils actuation status during measurement
      - Magnetometer measurement data (raw ADCS and calibrated values)

  - Last measured dipole values

An example response might look like this:

.. code-block:: json

    {
        "system_mode": "IDLE",
        "system_error": "no",
        "system_configured": "yes",
        "system_uptime": 14,
        "supply_voltage_digital_raw": 2742,
        "supply_voltage_analog_raw": 2743,
        "supply_current_digital_raw": 731,
        "supply_current_analog_raw": 167,
        "coil_current_x_raw": 2122,
        "coil_current_y_raw": 2124,
        "coil_current_z_raw": 2123,
        "coil_temp_x_raw": 2276,
        "coil_temp_y_raw": 2270,
        "coil_temp_z_raw": 2270,
        "mcu_temp_raw": 1262,
        "supply_voltage_digital_eng": 3348,
        "supply_voltage_analog_eng": 3348,
        "supply_current_digital_eng": 373,
        "supply_current_analog_eng": 77,
        "coil_current_x_eng": 2,
        "coil_current_y_eng": 2,
        "coil_current_z_eng": 2,
        "coil_temp_x_eng": 21,
        "coil_temp_y_eng": 22,
        "coil_temp_z_eng": 22,
        "mcu_temp_eng": 25,
        "detumble_calib_mtm_x": 0,
        "detumble_calib_mtm_y": 0,
        "detumble_calib_mtm_z": 0,
        "detumble_filter_mtm_x": 0,
        "detumble_filter_mtm_y": 0,
        "detumble_filter_mtm_z": 0,
        "detumble_bdot_x": 0,
        "detumble_bdot_y": 0,
        "detumble_bdot_z": 0,
        "detumble_dipole_x": 0,
        "detumble_dipole_y": 0,
        "detumble_dipole_z": 0,
        "detumble_cmd_current_x": 0,
        "detumble_cmd_current_y": 0,
        "detumble_cmd_current_z": 0,
        "detumble_coil_current_x": 0,
        "detumble_coil_current_y": 0,
        "detumble_coil_current_z": 0,
        "mtm_actuating": "no",
        "mtm_x_raw": -705,
        "mtm_y_raw": -2879,
        "mtm_z_raw": 3583,
        "mtm_x_calib": -5288,
        "mtm_y_calib": -21608,
        "mtm_z_calib": 26888,
        "dipole_x": 7,
        "dipole_y": 80,
        "dipole_z": -129
    }

  
Debug
~~~~~

The debug telemetry will contain the following information:

  - The current values of all iMTQ configuration options
  - The results of the last run self-test as triggered by ``k_adcs_run_test`` or ``k_imtq_start_test``

An example response might look like this:

.. code-block:: json

    {
        "system_mode": "IDLE",
        "system_error": "no",
        "system_configured": "yes",
        "system_uptime": 13,
        "0x2002": 0,
        "0x2003": 1,
        "0x2004": 2,
        "0x2005": 3,
        "0x2006": 1,
        "0x2007": 5,
        "0x2008": 0,
        "0x2009": 1,
        "0x200a": 2,
        "0xa001": 1,
        "0xa002": 0,
        "0xa003": 0,
        "0xa004": 0,
        "0xa005": 1,
        "0xa006": 0,
        "0xa007": 0,
        "0xa008": 0,
        "0xa009": 1,
        "0xa00a": 0,
        "0xa00b": 0,
        "0xa00c": 0,
        "0x301c": 1294,
        "0x301d": 1291,
        "0x301e": 1297,
        "0x301f": 1,
        "0x3020": 1,
        "0x3021": 25,
        "0x3022": 2,
        "0x3023": 2,
        "0x3024": 12,
        "0x3025": 1567,
        "0x3026": 1567,
        "0x3027": 1567,
        "0x3028": -10,
        "0x3029": -10,
        "0x302a": -10,
        "0x302b": 81,
        "0x302c": 81,
        "0x302d": 81,
        "0x2000": 1,
        "0xa000": -10000,
        "0xa00d": 5e-05,
        "0xa00e": 0.1,
        "0xa00f": 5,
        "0xa010": 5,
        "0xa011": 2,
        "0x4000": 0,
        "0x2001": 0,
        "0x5000": -2000,
        "0x5001": -2000,
        "0x5002": -800,
        "0x3000": -40,
        "0x3001": -20,
        "0x3002": 0,
        "0x3003": 20,
        "0x3004": 40,
        "0x3005": 60,
        "0x3006": 70,
        "0x3007": 546,
        "0x3008": 498,
        "0x3009": 452,
        "0x300a": 416,
        "0x300b": 389,
        "0x300c": 363,
        "0x300d": 352,
        "0x300e": 545,
        "0x300f": 496,
        "0x3010": 449,
        "0x3011": 414,
        "0x3012": 387,
        "0x3013": 362,
        "0x3014": 350,
        "0x3015": 1545,
        "0x3016": 1410,
        "0x3017": 1277,
        "0x3018": 1178,
        "0x3019": 1102,
        "0x301a": 1029,
        "0x301b": 999,
        "0x2800": 0,
        "0x2801": 70,
        "0x4800": 16,
        "0x6800": 7689,
        "tr_init_error": 0,
        "tr_init_mtm_raw_x": -755,
        "tr_init_mtm_raw_y": -2563,
        "tr_init_mtm_raw_z": 3749,
        "tr_init_mtm_calib_x": -5663,
        "tr_init_mtm_calib_y": -19238,
        "tr_init_mtm_calib_z": 28133,
        "tr_init_coil_current_x": -2,
        "tr_init_coil_current_y": -2,
        "tr_init_coil_current_z": -14,
        "tr_init_coil_temp_x": 21,
        "tr_init_coil_temp_y": 22,
        "tr_init_coil_temp_z": 22,
        "tr_posz_error": 96,
        "tr_posz_mtm_raw_x": -21355,
        "tr_posz_mtm_raw_y": -9991,
        "tr_posz_mtm_raw_z": 91997,
        "tr_posz_mtm_calib_x": -160163,
        "tr_posz_mtm_calib_y": -74948,
        "tr_posz_mtm_calib_z": 689993,
        "tr_posz_coil_current_x": -98,
        "tr_posz_coil_current_y": 46,
        "tr_posz_coil_current_z": 581,
        "tr_posz_coil_temp_x": 21,
        "tr_posz_coil_temp_y": 22,
        "tr_posz_coil_temp_z": 22,
        "tr_fina_error": 0,
        "tr_fina_mtm_raw_x": -729,
        "tr_fina_mtm_raw_y": -2789,
        "tr_fina_mtm_raw_z": 2801,
        "tr_fina_mtm_calib_x": -5468,
        "tr_fina_mtm_calib_y": -20933,
        "tr_fina_mtm_calib_z": 21023,
        "tr_fina_coil_current_x": -2,
        "tr_fina_coil_current_y": -5,
        "tr_fina_coil_current_z": -8,
        "tr_fina_coil_temp_x": 21,
        "tr_fina_coil_temp_y": 22,
        "tr_fina_coil_temp_z": 22
    }
  
Other Functions
---------------

    - :cpp:func:`k_adcs_reset` - (TC-OP-01) Trigger a software reset of the iMTQ
    - :cpp:func:`k_adcs_noop` - (TC-OP-02) Perform a no-op operation. Useful for verifying the iMTQ is online.
    - :cpp:func:`k_adcs_get_power_status` - Returns the uptime of the iMTQ, in seconds. Value is zero if the iMTQ is offline.

iMTQ-specific Functions
-----------------------

    - :cpp:func:`k_imtq_cancel` - (TC-OP-03) Cancel any ongoing actuation and switch to idle mode

Watchdog
~~~~~~~~

The iMTQ has a watchdog which will restart the system if it has not been fed within the required interval.

There are two provided functions to assist with its maintenance:

    - :cpp:func:`k_imtq_watchdog_start` - Start a thread to send a no-op command every (watchdog_interval/3) seconds to keep the watchdog from starving
    - :cpp:func:`k_imtq_watchdog_stop` - Terminate the watchdog thread

Configuration
~~~~~~~~~~~~~

Get Current Parameter Value
^^^^^^^^^^^^^^^^^^^^^^^^^^^

To check the current value of a configuration parameter, use the :cpp:func:`k_imtq_get_param` function.

The function takes two parameters:

    - The parameter to fetch
    - A pointer to an :cpp:type:`imtq_config_resp` structure where the command response containing the parameter value should be put

If the function returns successfully, the value can be read from the appropriate unit submember of the
:cpp:member:`imtq_config_resp.value <mtq_config_resp::value>` structure member.

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>

    KADCSStatus status = ADCS_OK;
    imtq_config_resp result;
    
    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize iMTQ connection\n");
        return;
    }

    status = k_imtq_get_param(SLAVE_ADDRESS, &result);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't get parameter value: %d\n", status);
        k_adcs_terminate();
        return status;
    }

    if (result.value.uint16_val == 0x10)
    {
        printf("Get parameter test completed successfully\n");
    }
    else
    {
        fprintf(stderr, "Parameter value mismatch - Expected: %x Received: %x\n", 
                        IMTQ_ADDR, result.value.uint16_val);
    }
    
    k_adcs_terminate();

Set New Parameter Value
^^^^^^^^^^^^^^^^^^^^^^^

.. warning::

    **These configuration changes will not persist through reboot, including one triggered by the watchdog**

To set a new value for a configuration parameter, use the :cpp:func:`k_imtq_set_param` function.

The function takes three parameters:

    - The parameter to fetch
    - A pointer to an :cpp:type:`imtq_config_value` structure containing the new parameter value
    - A pointer to an :cpp:type:`imtq_config_resp` structure where the command response containing the updated parameter value should be put

If the function returns successfully, the updated value can be read from the appropriate unit submember of the
:cpp:member:`imtq_config_resp.value <mtq_config_resp::value>` structure member to verify that the parameter was updated as expected.

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>

    KADCSStatus status = ADCS_OK;
    imtq_config_resp result;
    imtq_config_value request;

    request.int16_val = 1300;
    
    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize iMTQ connection\n");
        return;
    }
    
    /* Set the new value */
    status = k_imtq_set_param(ADC_COIL_CURRENT_BIAS_X, &request, &result);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to set parameter value: %d\n", status);
        return status;
    }

    /* Verify that it was set successfully */
    if (request.int16_val != result.value.int16_val)
    {
        fprintf(stderr, "Unable to change parameter: %d %d\n", 
                        request.int16_val, result.value.int16_val);
    }
    else
    {
        printf("Set parameter test completed successfully\n");
    }
    
    k_adcs_terminate();

Reset Parameter Value
^^^^^^^^^^^^^^^^^^^^^

To reset a configuration parameter to its default value, use the :cpp:func:`k_imtq_reset_param` function.

The function takes two parameters:

    - The parameter to fetch
    - (optional) A pointer to an :cpp:type:`imtq_config_resp` structure where the command response containing the parameter value should be put

If the function returns successfully, the value can be read from the appropriate unit submember of the
:cpp:member:`imtq_config_resp.value <mtq_config_resp::value>` structure member.
This value should match the default documented in section 3.4 of the *iMTQ User Manual*.

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>
    
    KADCSStatus status = ADCS_OK;
    imtq_config_resp result;
    
    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize iMTQ connection\n");
        return;
    }
    
    status = k_imtq_reset_param(CURRENT_FEEDBACK_ENABLE, &result);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to reset parameter: %d\n", status);
        return status;
    }
    
    if (result.value.uint8_val != 0)
    {
        fprintf(stderr, "Returned value does not match the default: %d\n", result.value.uint8_val);
    }
    
    k_adcs_terminate();
           
Magnetometer Measurements
~~~~~~~~~~~~~~~~~~~~~~~~~

The :cpp:func:`k_imtq_start_measurement` function should be called to request that the iMTQ start a 3-axis measurement
of the magnetic field.
This corresponds with command TC-OP-04 documented in the *iMTQ User Manual*.

After this function has been called, either :cpp:func:`k_imtq_get_raw_mtm` or :cpp:func:`k_imtq_get_calib_mtm` can be
called in order to read the results of the measurement.
These functions correspond with commands TC-DR-02 and TC-DR-03 documented in the *iMTQ User Manual*.

These functions take a pointer to an :cpp:type:`imtq_mtm_msg` structure where the command response containing the
measurement data should be put.

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>
    
    KADCSStatus status = ADCS_OK;
    
    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize iMTQ connection\n");
        return;
    }

    /* Request that the iMTQ get MTM measurements */
    status = k_imtq_start_measurement();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start MTM measurement: %d\n", status);
        return status;
    }

    /* Give it a little time to complete the request */
    const struct timespec TRANSFER_DELAY = {
        .tv_sec = 0,
        .tv_nsec = 2000000
    };
    nanosleep(&TRANSFER_DELAY, NULL);

    /* Read the measurements */
    imtq_mtm_msg mtm = {0};

    status = k_imtq_get_raw_mtm(&mtm);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get calib measurement: %d\n", status);
        return status;
    }

    printf("MTM Measurement (raw) - X: %10d Y: %10d Z: %10d Actuating: %s\n",
            mtm.data.x, mtm.data.y, mtm.data.z, mtm.act_status ? "yes" : "no");

    status = k_imtq_get_calib_mtm(&mtm);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get calib measurement: %d\n", status);
        return status;
    }

    printf("MTM Measurement (cal) - X: %10d Y: %10d Z: %10d Actuating: %s\n",
            mtm.data.x, mtm.data.y, mtm.data.z, mtm.act_status ? "yes" : "no");
            
    k_adcs_terminate();
    
Actuation
~~~~~~~~~

As documented in section 2.6 of the *iMTQ User Manual*, the iMTQ's coils can be used to generate a dipole.

This can be done by using one of three functions:

    - :cpp:func:`k_imtq_start_actuation_current` - (TC-OP-05) Actuate using desired coil currents
    - :cpp:func:`k_imtq_start_actuation_dipole` - (TC-OP-06) Actuate using desired dipole
    - :cpp:func:`k_imtq_start_actuation_PWM` - (TC-OP-07) Actuate using desired PWM duty cycle

.. note::

    Please refer to the corresponding command documentation in section 3.3 of the *iMTQ User Manual* for more
    information about the purpose and characteristics of these functions
    
Each of these functions takes two parameters:

  - A pointer to an :cpp:type:`imtq_axis_data` structure containing the desired actuation value for the x-, y-, and z- axis
  - The duration, in milliseconds, which the actuation should be active for

.. note::

    The :cpp:func:`k_imtq_start_actuation_current` and :cpp:func:`k_imtq_start_actuation_PWM` functions both
    have limits on what the actuation values for each axis can be. See the function API documentation for
    more information
    
For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>
    
    KADCSStatus status = ADCS_OK;
    imtq_axis_data currents = {0};
    uint16_t time = 10;

    currents.x = -410;
    currents.y = -408;
    currents.z = -1159;
    
    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize iMTQ connection\n");
        return;
    }

    status = k_imtq_start_actuation_current(currents, time);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start actuation (current): %d\n", status);
        return status;
    }
    
    k_adcs_terminate();
     
Detumble
~~~~~~~~

The :cpp:func:`k_imtq_start_detumble` function can be used to switch the iMTQ into detumble mode for a desired duration.

This corresponds with the TC-OP-09 command documented in section 3.3 of the *iMTQ User Manual*

The function takes a single parameter indicating the time, in seconds, that the iMTQ should be in detumble mode before
returning to idle mode.

After this function has run, the :cpp:func:`k_imtq_get_detumble` function can be used to fetch the measurements and
computations generated while the iMTQ was in detumble mode.

This corresponds with the TC-DR-08 command documented in section 3.3 of the *iMTQ User Manual*

The function takes a pointer to an :cpp:type:`imtq_detumble` structure where the command response containing the generated
data should be put.

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>

    KADCSStatus status = ADCS_OK;
    uint16_t time = 0;
    
    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize iMTQ connection\n");
        return;
    }

    /* Enter detumble mode */
    status = k_imtq_start_detumble(time);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to start actuation (dipole): %d\n", status);
        return status;
    }
    
    /* Give it a little time to complete the request */
    const struct timespec TRANSFER_DELAY = {
        .tv_sec = 0,
        .tv_nsec = 2000000
    };
    nanosleep(&TRANSFER_DELAY, NULL);
    
    /* Fetch the data */
    imtq_detumble data = {0};

    status = k_imtq_get_detumble(&data);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get detumble data: %d\n", status);
        return status;
    }

    printf("Detumble Data: \n"
            "------------------------------\n\n");
    printf("Calibrated - X: %d Y: %d Z: %d\n",
            data.mtm_calib.x, data.mtm_calib.y, data.mtm_calib.z);
    printf("Filtered - X: %d Y: %d Z: %d\n",
            data.mtm_filter.x, data.mtm_filter.y, data.mtm_filter.z);
    printf("B-dot - X: %d Y: %d Z: %d\n",
            data.bdot.x, data.bdot.y, data.bdot.z);
    printf("Command actuation dipole - X: %d Y: %d Z: %d\n",
                data.dipole.x, data.dipole.y, data.dipole.z);
    printf("Command current - X: %d Y: %d Z: %d\n",
                data.cmd_current.x, data.cmd_current.y, data.cmd_current.z);
    printf("Coil current - X: %d Y: %d Z: %d\n",
                data.coil_current.x, data.coil_current.y, data.coil_current.z);
                
    k_adcs_terminate();
    
      
Telemetry
~~~~~~~~~

There are several iMTQ-specific functions which can also be used to get current data about the system.

System State
^^^^^^^^^^^^

The :cpp:func:`k_imtq_get_system_state` function can be used to check the current state of the iMTQ.

This corresponds with the TC-DR-01 command documented in section 3.3 of the *iMTQ User Manual*

The function takes a pointer to an :cpp:type:`imtq_state` structure where the command response containing the system information
should be put.

Coil Current
^^^^^^^^^^^^

The :cpp:func:`k_imtq_get_coil_current` function can be used to fetch the current measurement of the iMTQ's coils.

This corresponds with the TC-DR-04 command documented in section 3.3 of the *iMTQ User Manual*

The function takes a pointer to an :cpp:type:`imtq_coil_current` structure where the command response containing the current for
each of the axes should be put.

Coil Temperatures
^^^^^^^^^^^^^^^^^

The :cpp:func:`k_imtq_get_coil_temps` function can be used to fetch the temperature measurement of the iMTQ's coils.

This corresponds with the TC-DR-05 command documented in section 3.3 of the *iMTQ User Manual*

The function takes a pointer to an :cpp:type:`imtq_coil_temp` structure where the command response containing the temperature for
each of the axes should be put.

Dipole
^^^^^^

The :cpp:func:`k_imtq_get_dipole` function can be used to fetch the commanded actuation dipole.

This corresponds with the TC-DR-04 command documented in section 3.3 of the *iMTQ User Manual*

.. note:: 

    This function will return the values generated after running either the :cpp:func:`k_imtq_start_actuation_dipole` or
    :cpp:func:`k_imtq_start_detumble` functions.

The function takes a pointer to an :cpp:type:`imtq_dipole` structure where the command response containing the dipole for
each of the axes should be put.

Housekeeping Data
^^^^^^^^^^^^^^^^^

The :cpp:func:`k_imtq_get_raw_housekeeping` and :cpp:func:`k_imtq_get_eng_housekeeping` functions can be used to fetch
the housekeeping data of the iMTQ, containing information like system voltages, currents, and temperatures.

These functions correspond with commands TC-DR-09 and TC-DR-10 documented in the *iMTQ User Manual*.

The :cpp:func:`k_imtq_get_raw_housekeeping` function takes a pointer to a :cpp:type:`imtq_housekeeping_raw` structure and returns
the raw ADC counts for each of the telemetry items.

The :cpp:func:`k_imtq_get_eng_housekeeping` function takes a pointer to a :cpp:type:`imtq_housekeeping_eng` structure and returns
the converted engineering values for each of the telemetry items.

For example:

.. code-block:: c

    #include <isis-imtq-api/imtqh>
    
    KADCSStatus status = ADCS_OK;
    imtq_housekeeping_eng data = {0};

    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize iMTQ connection\n");
        return;
    }

    status = k_imtq_get_eng_housekeeping(&data);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get eng housekeeping data: %d\n", status);
        return status;
    }

    printf("Housekeeping Data (eng): \n"
            "------------------------------\n\n");
    printf("Voltage - D: %d A: %d\n", data.voltage_d, data.voltage_a);
    printf("Current - D: %d A: %d\n", data.current_d, data.current_a);
    printf("Coil current - X: %d Y: %d Z: %d\n",
                data.coil_current.x, data.coil_current.y, data.coil_current.z);
    printf("Coil temp - X: %d Y: %d Z: %d\n",
                data.coil_temp.x, data.coil_temp.y, data.coil_temp.z);
    printf("MCU temp - %d\n", data.mcu_temp);

    k_adcs_terminate();
    
