HTU21D Sensor API
=================

`HTU21D humidity sensor <https://cdn-shop.adafruit.com/datasheets/1899_HTU21D.pdf>`__

Enabling this sensor code requires certain configuration values to be present
in the application's configuration json. An example is given below:

::

     {
         "sensors": {
             "htu21d": {
                 "i2c_bus": "K_I2C1"
             }
         }
     }
     
See the :ref:`i2c-example` for an example of how to use this sensor.

.. doxygengroup:: KUBOS_CORE_HTU21D
    :project: kubos-core
    :members:
    :content-only: 