Kubos Core Altimeter API
========================

Enabling this sensor code requires certain configuration values to be present
in the application's configuration json. An example is given below:

::

     {
         "sensors": {
             "bme280": {
                 "spi_bus": "K_SPI1"
             }
         }
     }

This would enable the sensor API and the bme280 sensor code and configure
it for the SPI bus K_SPI1.

.. doxygengroup:: KUBOS_CORE_ALTIMETER
    :project: kubos-core
    :members:
    :content-only: 