IMU API
=======

The IMU API abstracts away the need to specify a specific sensor's init/read/write/etc functions
in order to gather positional data.

In order to use this API, the :json:object:`sensors` object must be present in the project's `config.json` file,
along with one of the supported child sensors. An example is given below:

::

     {
         "sensors": {
             "bno055": {
                 "i2c_bus": "K_I2C1"
             }
         }
     }
 
This would enable the sensor API and the BNO055 sensor code and configure
it for the I2C bus K_I2C1.

.. doxygengroup:: KUBOS_CORE_IMU
    :project: kubos-core
    :members:
    :content-only: 