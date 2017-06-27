Temperature API
===============

The temperature API abstracts away the need to specify a specific sensor's init/read/write/etc functions
in order to gather temperature and humidity data.

In order to use this API, the :json:object:`sensors` object must be present in the project's `config.json` file,
along with one of the supported child sensors. An example is given below:

::

     {
         "sensors": {
             "htu21d": {
                 "i2c_bus": "K_I2C1"
             }
         }
     }
     
See the :ref:`sensor-example` for an example of how to use this sensor.

.. doxygengroup:: KUBOS_CORE_TEMPERATURE
    :project: kubos-core
    :members:
    :content-only: 