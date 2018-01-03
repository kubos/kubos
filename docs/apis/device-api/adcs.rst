Using an ADCS
=============

This document covers the general API for interacting with a supported ADCS. The underlying structure for ADCSs varies
quite dramatically, so please refer to the appropriate specific ADCS document in conjunction with this one.

The specific ADCS documents will cover things like configuration options, available telemetry types, and system requirements.

Project Configuration
---------------------

The specific ADCS being used in a Kubos project should be specified in the ``adcs`` parameter of the project's `config.json` file.
This tells the Kubos SDK to include the correct ADCS files when building the project.
The ADCS's subparameters may then be specified within the specific ADCS's structure.

For example::

    {
      "adcs": {
        "imtq": {
          "watchdog": {
            "timeout": 3600
          }
        }
      }
    }
    
ADCS Initialization and Termination
-----------------------------------

In order for a Kubos project to communicate with a ADCS, the ``k_adcs_init`` function should be called. This opens the correct
underlying KubOS Linux device file. The function should return ``ADCS_OK`` if it completed successfully.

Similarly, before the project finishes executing, the ``k_adcs_terminate`` function should be called to perform the appropriate
system cleanup.

For example:

.. code-block:: c

    #include "adcs-api/adcs.h"
    
    if (k_adcs_init() != ADCS_OK)
    {
        return;
    }
    
    // Project logic
    
    k_adcs_terminate();

Run-Time ADCS Configuration
---------------------------

Some ADCSs may support or require that the ADCS be configured during run-time. This can be done by passing a pointer to 
a JSON structure (defined as ``JsonNode *``) to the ``k_adcs_configure`` function.

The JSON structure should be created using the CCAN/JSON library in the Kubos repo.

The user may either convert a character buffer containing JSON using ``json_decode`` and/or create a JSON structure manually
using ``json_mkobject`` and ``json_append_member``.

The passed JSON should be a flat structure consisting solely of name/value pairs.

.. warning:: The JSON structure used for configuration must be deleted **by the user** once it is no longer of use.

For example:

.. code-block:: c

    #include <adcs-api/adcs.h>
    
    KADCSStatus status = ADCS_OK;
  
    status = k_adcs_init();
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Couldn't initialize ADCS connection\n");
        return;
    }
    
    /* Create the JSON structure with our desired configuration options */
    JsonNode * config = json_decode("{\"0x2003\": 1,   \"0x2004\": 2}");
    json_append_member(config, "0x2005", json_mknumber(4));
    
    /* Configure the iMTQ */
    status = k_adcs_configure(config);
    
    /* Delete the JSON structure now that we're done with it */
    json_delete(config);
    
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to configure ADCS\n");
    }

    k_adcs_terminate();
    
Self-Tests
----------

Some ADCSs are capable of running self-diagnostics. These self-tests are executed using :cpp:func:`k_adcs_run_test`.

The available self-tests are documented in the specific ADCS API.

This function takes two parameters:

    - The self-test to run
    - A pointer to a JSON structure to which the test results should be added

.. warning:: The JSON structure used for the self-test results must be deleted **by the user** once it is no longer of use.

For example:

.. code-block:: c

    #include <adcs-api/adcs.h>
    
    KADCSStatus status = ADCS_OK;

    /* Make parent object */
    adcs_test_results test = json_mkobject();

    /* Get the data */
    status = k_adcs_run_test(TEST_ALL, test);
    if (status != ADCS_OK)
    {
        fprintf(stderr, "Failed to get iMTQ telemetry\n");
        json_delete(test);
        return ADCS_ERROR;
    }

    /* Print results */
    char * temp = json_stringify(test, " ");
    puts(temp);
    free(temp);

    /* Free the memory */
    json_delete(test);

Telemetry
---------

The :cpp:func:`k_adcs_get_telemetry` function can be used to fetch current telemetry information about the ADCS.

The available types of telemetry are documented in the specific ADCS API.

This function takes two parameters:

    - The type of telemetry to return
    - A pointer to a JSON structure (defined as ``JsonNode *``) to which the telemetry results should be added

.. warning:: The JSON structure used for the telemetry information must be deleted **by the user** once it is no longer of use. 

For example:

.. code-block:: c

    #include <adcs-api/adcs.h>
    
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





