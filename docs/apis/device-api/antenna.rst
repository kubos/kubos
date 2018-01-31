Using an Antenna
================

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
    