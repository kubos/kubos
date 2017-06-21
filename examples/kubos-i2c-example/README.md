# Kubos I2C Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/KubOS-rt) demonstrating our I2C HAL.

This application gives several examples of how to interact with I2C devices in a Kubos project:

If no sensor has been defined in the project’s config.json file, then this application will initialize a generic I2C connection over I2C bus 1 to a slave device with an address of ‘0x40’.
  - It will then write a single byte command of ‘0xE3’ to the slave and attempt to read back a three byte response.
  - After this attempt, the application will end.
  
If the HTU21D sensor has been defined in the project’s config.json file, the appropriate initialization calls will be made and then the application will enter a loop.
  - In each iteration of the loop, the application will fetch and print the current temperature and humidity data from the sensor to the default UART port.
  
If the BNO055 sensor has been defined in the project’s config.json file, the sensor will be initialized in NDOF (Nine Degrees Of Freedom) mode and then the application will enter a loop.
  - In each iteration of the loop, the application will fetch and print the current position data from the sensor to the default UART port.

The easiest way to get started building this is with the [Kubos SDK](http://docs.kubos.co/latest/md_docs_kubos-sdk.html).
