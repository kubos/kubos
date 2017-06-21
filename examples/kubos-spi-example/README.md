# Kubos SPI Example App

This is a simple application built on top of the [KubOS RT Platform](https://github.com/kubostech/KubOS-rt) demonstrating our SPI HAL.

This application gives two examples of how to interact with SPI devices in a Kubos project:

If no sensor has been defined in the project’s config.json file, then this application will initialize a generic SPI connection over SPI bus 1.
  - The application will then enter a loop and attempt to send and receive a dummy byte.
  - **Note**: This case is not a complete example, because it omits the manual manipulation of a chip select pin that is required for SPI communication.
  
If the BME280 sensor has been defined in the project’s config.json file, the appropriate initialization calls will be made and then the application will enter a loop.
  - In each iteration of the loop, the application will fetch and print the current temperature and humidity data from the sensor to the default UART port.

The easiest way to get started building this is with the [Kubos SDK](http://docs.kubos.co/latest/md_docs_kubos-sdk.html).
