# KubOS Linux  - LSM303DLHC I2C Test

This project tests communication with the [LSM303DLHC compass sensor](http://www.st.com/content/ccc/resource/technical/document/datasheet/56/ec/ac/de/28/21/4d/48/DM00027543.pdf/files/DM00027543.pdf/jcr:content/translations/en.DM00027543.pd) over I2C.

This integration test:
- Connects to I2C bus 0
- Connects to the LSM303DLHC sensor at device address 0x19
- Sets the operation mode of the sensor
- Gets the operation mode of the sensor
- Checks that what was sent and gotten match

Test Shell Command:

    $ lsm303dlhc-i2c

Expected Output:

    LSM303DLHC I2C test completed successfully!