Altimeter API
=============

The altimeter API abstracts away the need to specify a specific sensor's init/read/write/etc functions
in order to gather pressure and altitude data.

In order to use this API, the :json:object:`sensors` object must be present in the project's `config.json` file,
along with one of the supported child sensors. An example is given below:

::

     {
         "sensors": {
             "bme280": {
                 "spi_bus": "K_SPI1"
             }
         }
     }

This would enable the sensor API and the BME280 sensor code and configure
it for the SPI bus K_SPI1.

See the :ref:`sensor-example` for an example of how to use this interface.

.. doxygengroup:: KUBOS_CORE_ALTIMETER
    :project: kubos-core
    :members:
    :content-only: 