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

## Build Environment (with Docker)

1. Install Docker(https://docs.docker.com/engine/installation/)
2. Ensure the docker daemon is running.
3. Create a Docker image from our Dockerfile

        $ docker build -t kubos-image dist/

4. Start a Kubos Docker instance (in the background)

        $ docker run --name kubos-inst -d=True -v /home/kubos/KubOS/:/build/dev kubos-image

5. Start up a shell on the instance

        $ docker exec -t -i kubos-inst bash -l

6. This docker image already has yotta and the neccesary toolchains installed!

## Build Environment (local)

1. Install ARM's [yotta build system](http://yottadocs.mbed.com/#installing)
2. Install CMake 3.x
3. Install the [ARM GCC toolchain](https://github.com/RIOT-OS/RIOT/wiki/Family:-ARM)
4. Follow the target-specific instructions below

### STM32F407 Discovery

1. Clone our top level [Kubos project](https://github.com/openkosmosorg/KubOS)

2. Bootstrap our projects (this will also link the local yotta modules)

        $ cd KubOS

        $ ./bootstrap.sh

3. Navigate to our example app

        $ cd examples/kubos-rt-example

4. Install our custom `stm32f407-disco-gcc`

        $ yotta target stm32f407-disco-gcc

5. Build the example app

        $ yotta build -- -v
