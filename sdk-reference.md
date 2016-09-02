# KubOS-SDK Command Reference

## Creating a project

Run the `kubos init` command followed by the name of your project to bootstrap your KubOS project. This will create a new directory under your current working directory with your project's name and add the basic files.

        $ kubos init project-name

Note - Inside of the build system there are several reserved words, a project cannot be named any of these words. These are `test`, `source`, `include`, `yotta_modules` and `yotta_targets`.

The contents of your project directory should look something like this:

        $ ls
        module.json  project-name  source  test

Here is a quick rundown of the files that were generated:

| File/folder   | Description  |
| ------------- |-------------|
| `project-name` | This folder is where header files live |
| `source`   | This folder is where source files live |
| `test`    | This folder is where test source files live |
| `module.json` | This file is yotta's module description file |


KubOS uses the yotta build/module system, which is where this file structure comes from. You can read more about yotta [here](http://yottadocs.mbed.com/).

## Selecting a target

Yotta needs to know which target you intend to build for so it can select the proper cross compiler. KubOS currently supports several different targets:

| MCU Family   | Board  |
| ------------- |-------------|
| STM32F4 | STM32F407 Discovery Board |
|    |  STM32F405 PyBoard |
|  | STM32F405 NanoAvionics SatBus 3C0 OBC |
| MSP430     | MSP430F5529 Launchpad |


The respective commands to select those targets are as follows.

        $ kubos target stm32f407-disco-gcc

        $ kubos target pyboard-gcc

        $ kubos target na-satbus-3c0-gcc

        $ kubos target msp430f5529-gcc

If you have no target currently set, you may run the following command to see which targets are available:

        $ kubos target

## Building a project

To build a KubOS project, all we need to do is run the `kubos build` command. The KubOS SDK (really `yotta` under the covers) will read the module.json file, determine what libraries are needed and build them.

Basic build command:

        $ kubos build

Build with verbose output:

        $ kubos build -- -v

Clean command:

        $ kubos clean

## Linking local modules & targets

The KubOS SDK comes with all of the latest KubOS modules and targets pre-packaged and pre-linked. If a module or target needs to be modified locally, the SDK comes with the ability to link that local module into the build process.

##### Linking modules:

 * Links are made in two steps - first globally then locally.

 * By linking a module globally you are making it available to link into any of your projects. By linking the module locally you are including the linked module in your build.

 * To link a module globally:

        $ cd .../<module-directory>/
        $ kubos link

 * To link a module that is already globally linked into a project:

        $ cd .../<project-directory>/
        $ kubos link <module name>

 * To link a module directly into a project in one step:

        $ cd .../<project-directory>/
        $ kubos link /path/to/module/

 * By doing this in one step kubos automatically links the module globally and then links it into your local project for you.

The next time your project is built it will use your local development module, rather than the packaged version.

##### Linking targets:

 * Custom or modified targets are linked in a very similar way to modules.

 * Links are made in two steps - first globally then locally.

 * By linking a target globally you are making it available to link into any of your projects. By linking the target locally you are now able to use the linked target in your build.

 * To link a target globally:

        $ cd .../<target-directory>/
        $ kubos link-target

 * To link a target that is already globally linked into a project:

        $ cd .../<project-directory>/
        $ kubos link-target <target name>

 * You may now use the standard target command to select the newly linked target:

        $ cdc ../<project-directory>/
        $ kubos target <target name>

The next time your project is built it will use your local development target, rather than the packaged version.


## Flashing your project

Flashing your project using the kubos tool is a relatively straightforward process:

1. Ensure that your board is plugged into your computer

2. Run the flash command

        $ kubos flash

*Note: If your current user does not have read/write permission to your hardware device you may need to run this command as root*

        $ sudo kubos flash

#### Debug your project

A gdb server must be started to allow your gdb instance to connect and debug directly on your hardware device.
After building your project with `kubos build` kubos can manage a gdb server and gdb instance for you.

Start a gdb server and instance for you:
Note: this may need to run as root depending on your usb device permissions

        $ kubos debug

Additionally you can interact directly with the gdb server:

        $ kubos server <start, stop, restart, status>
