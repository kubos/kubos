# KubOS RT
Real-time OS for constrained satellite subsystems

KubOS RT bundles and extends several open source projects to enable rapid
development on satellite hardware:

* [FreeRTOS](http://freertos.org)
* [CubeSat Space Protocol](http://github.com/GOMspace/libcsp)
* [KubOS Core Flight Middleware](http://github.com/openkosmosorg/kubos-core)
* [KubOS HAL](http://github.com/openkosmosorg/kubos-hal)

For our v1.0 release, we will be targetting these CubeSat OBCs, and associated development boards:

* [NanoAvionics SatBus 3C0](http://n-avionics.com/command-service-modules) / STM32F407 Discovery
* [PocketQube Shop OBC](http://www.pocketqubeshop.com/hardware/on-board-computer) / MSP430F5529 Launchpad
* [ISIS ARM9 OBC](http://www.cubesatshop.com/index.php?page=shop.product_details&flypage=flypage.tpl&product_id=119&category_id=8&option=com_virtuemart&Itemid=75&vmcchk=1&Itemid=75)

## Building

1. Install ARM's [yotta build system](http://yottadocs.mbed.com/#installing)
2. Follow the target-specific instructions below

### STM32F407 Discovery

1. Clone our [example kubos-rt app](https://github.com/openkosmosorg/kubos-rt-example)

1. Install our custom `stm32f407-disco-gcc` target from Github (this will be cleaner soon)

        $ yotta target stm32f407-disco-gcc@openkosmosorg/target-stm32f407-disco-gcc#master

2. Build the example app

        $ yotta build -- -v
