# KubOS Linux  - BME280 SPI Test

This project tests communication with the [BME280 humidity and pressure sensor](https://cdn-shop.adafruit.com/datasheets/BST-BME280_DS001-10.pdf) over SPI.

This integration test:
- Connects to SPI bus 1
- Connects to the BME280 sensor on the requested chip select pin
- Triggers a software reset of the chip
- Reads the chip ID and verifies it matches the expected value

Test Shell Command:

    $ bme280-spi [chip-select]

Expected Output:

    BME280 SPI test completed successfully!