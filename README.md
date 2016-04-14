## Building

1. Install ARM's [yotta build system](http://yottadocs.mbed.com/#installing)
2. Follow the target-specific instructions below

### STM32F407 Discovery

1. Install our custom `stm32f407-disco-gcc` target from Github (this will be cleaner soon)

        $ yotta target stm32f407-disco-gcc@openkosmosorg/target-stm32f407-disco-gcc#master

2. Build the example app included with KubOS RT

        $ yotta build -- -v
