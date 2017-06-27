Kubos Core IMU API
==================


Enabling this sensor code requires certain configuration values to be present
in the application's configuration json. An example is given below:

::

     {
         "sensors": {
             "bno055": {
                 "i2c_bus": "K_I2C1"
             }
         }
     }
 
This would enable the sensor API and the bno055 sensor code and configure
it for the I2C bus K_I2C1.

.. doxygengroup:: KUBOS_CORE_IMU
    :project: kubos-core
    :members:
    :content-only: 