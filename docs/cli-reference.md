# Kubos CLI Command Reference

## Common Commands

[build](#building-a-project)

[clean](#cleaning-a-project)

[debug](#debugging-a-project)

[init](#creating-a-project)

[link](#linking-modules)

[link-target](#linking-targets)

[list || ls](#listing-dependencies)

[start || flash](#flashing-a-project)

[target](#selecting-a-target)

[update](#updating-kubos)

[version](#checking-the-kubos-version)

## Other Commands

[config](#checking-a-configuration)

[licenses](#displaying-licenses)

[outdated](#checking-for-old-modules)

[remove](#removing-dependency-files)

[shrinkwrap](#freezing-dependency-versions)

[test](#testing-a-project)

[use](#updating-kubos)

[versions](#listing-available-kubos-versions)


## Building a Project

    kubos build [-h] [--config path/to/config.json] [-g] [-r] [-d]
                     [-G CMAKE_GENERATOR]
                     [MODULE_TO_BUILD [MODULE_TO_BUILD ...]]
                     
The programs or libraries to build can be specified (by default only the libraries needed by the current module and the current module's own tests are built). For example, to build the tests of all dependencies, run:
  yotta build all_tests

Positional arguments:

    MODULE_TO_BUILD       List modules or programs to build (omit to build the default set, or use "all_tests" to build all tests, including those of dependencies).

Optional arguments:

    -h, --help            show help message
    --config path/to/config.json
                        Specify the path to a JSON configuration file to extend the build configuration provided by the target
    -g, --generate-only   Only generate CMakeLists, don't run CMake or build
    -r, --release-build
    -d, --debug-build
    -G CMAKE_GENERATOR, --cmake-generator CMAKE_GENERATOR
                            CMake generator to use (defaults to Ninja). You can use this to generate IDE project files instead, see cmake --help for possible generator names. Note that only Ninja or Unix Makefile based generators will work correctly with yotta.


To build a Kubos project, all we need to do is run the `kubos build` command. The Kubos CLI will read the module.json file, determine what libraries are needed and build them.

Basic build command:

        $ kubos build

Build with verbose output:

        $ kubos build -- -v
        
(Options can be passed to the underlying build tool by passing them after --)

## Cleaning a Project

    kubos clean [-h]

Optional arguments:
    
    -h, --help  show help message

This command removes the build folder that is created with `kubos build` so that a completely fresh build can then be created.

## Checking a Configuration

    kubos config [-h] [--config path/to/config.json] [--plain]
                    [--colourful]

Optional arguments:

    -h, --help            show help message
    --config path/to/config.json
                           Specify the path to a JSON configuration file to extend the build configuration provided by the target
    --plain               Use a simple output format with no colours.
    --colourful           Force colourful output, even if the output is not to a tty.

If you want to see what options your project is being built with, run the `kubos config` command.  It will display the combined JSON values of both the default configuration for the target and any user configuration options specified in the project's config.json file.

## Debugging a Project

    kubos debug [-h] [--config path/to/config.json] [program]

Positional arguments:

    program               name of the program to be debugged

Optional arguments:

    -h, --help            show help message
    --config path/to/config.json
                        Specify the path to a JSON configuration file to extend the build configuration provided by the target.

A gdb server must be started to allow your gdb instance to connect and debug directly on your hardware device.
After building your project with `kubos build` the Kubos CLI can start a gdb server and gdb instance for you.

Start a gdb server and instance:
Note: this may need to run as root depending on your usb device permissions

        $ kubos debug


## Creating a Project

    kubos init [-h] [-l | -r] proj_name
    
Positional arguments:

    proj_name    specify the project name

Optional arguments:

      -h, --help   show help message
      -l, --linux  Initialize Kubos SDK project for KubOS Linux
      -r, --rt     Initialize Kubos SDK project for KubOS RT


Run the `kubos init` command followed by the name of your project to bootstrap your Kubos project. This will create a new directory under your current working directory with your project's name and add the source files for a basic Kubos project (kubos-rt-example).

        $ kubos init project-name

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


Kubos uses the yotta build/module system, which is where this file structure comes from. You can read more about yotta [here](http://yottadocs.mbed.com/).

### Note: Reserved Project Names

Inside of the build system there are several reserved words which cannot be used as the Kubos project name:

- `test`
- `source`
- `include`
- `yotta_modules`
- `yotta_targets`

## Displaying Licenses

    kubos licenses [-h] [--all]
    
optional arguments:

    -h, --help  show help message
    --all, -a   List all licenses, not just each unique license.

If you'd like to see which licenses Kubos is currently using, you can use the `kubos licenses` command.

## Linking Modules

    kubos link [-h] [-a] [module_or_path]

Positional arguments:
    
    module_or_path  Link a globally installed (or globally linked) module into the current module's dependencies. If ommited, globally link the current module.

Optional arguments:

    -h, --help      show help message
    -a, --all       Link all modules (and targets) from the global cache into the local project.

Links are made in two steps - first globally then locally.

By linking a module globally you are making it available to link into any of your projects. By linking the module locally you are including the linked module in your build.

**Note:** In order to be able to be linked, the module must have a module.json file.

To link a module globally:

        $ cd .../<module-directory>/
        $ kubos link

To link a module that is already globally linked into a project:

        $ cd .../<project-directory>/
        $ kubos link <module name>

The next time your project is built it will use your local development module, rather than the packaged version.

To verify where all of your targets are being loaded from `kubos list` will show you which modules are linked and which are local to your project

## Linking Targets

    kubos link-target [-h] [target_or_path]
    
Positional arguments:

    target_or_path  Link a globally installed (or globally linked) target into the current target's dependencies. If ommited, globally link the current target.

Optional arguments:

    -h, --help      show help message

Custom or modified targets are linked in a very similar way to modules.

Links are made in two steps - first globally then locally.

By linking a target globally you are making it available to link into any of your projects. By linking the target locally you are now able to use the linked target in your build.

To link a target globally:

        $ cd .../<target-directory>/
        $ kubos link-target

To link a target that is already globally linked into a project:

        $ cd .../<project-directory>/
        $ kubos link-target <target name>

You may now use the standard target command to select the newly linked target:

        $ cd ../<project-directory>/
        $ kubos target <target name>

The next time your project is built it will use your local development target, rather than the packaged version.

Running `kubos target` will show you whether you are using a local or a linked copy of a target

## Listing Dependencies

    kubos list [-h] [--config path/to/config.json] [--all]
                  [--display-origin] [--json]
                  
    kubos ls [-h] [--config path/to/config.json] [--all]
                  [--display-origin] [--json]

Optional arguments:

    -h, --help            show help message
    --config path/to/config.json
                            Specify the path to a JSON configuration file to extend the build configuration provided by the target
    --all, -a             Show all dependencies (including repeats, and test-only dependencies)
    --display-origin, -i  Display where modules were originally downloaded from (implied by --all).
    --json, -j            Output json representation of dependencies (implies --all).

Use the `kubos list` command to see a project's dependencies.  These are derived from the project's module.json "dependencies" section and the dependency sections of each of those modules and so forth.
This command is also useful for seeing the source folder for each dependency and whether or not an external location is being used for a dependency of the project.

## Checking for Old Modules

    kubos outdated [-h]

Optional arguments:

    -h, --help  show help message

Display information about dependencies which have newer versions available.

## Removing Dependency Files

    kubos remove [-h] [<module>]
    
Positional arguments:

    <module>    Name of the module to remove. If omitted the current module or target will be removed from the global linking directory.

Optional arguments:

    -h, --help  show help message

Remove the downloaded version of a dependency module or target, or un-link a linked module or target. This command does not remove the dependency as a requirement for your project.
In order to remove the dependency entirely, you'll need to update the module.json file and remove the module from the dependencies list.

## Freezing Dependency Versions

    kubos shrinkwrap [-h]

Optional arguments:

    -h, --help  show help message

Create a yotta-shrinkwrap.json file to freeze dependency versions.

## Testing a Project

    kubos test [-h] [--config path/to/config.json] [--list] [--no-build]
                  [-r] [-d] [-G CMAKE_GENERATOR]
                  [TEST_TO_RUN [TEST_TO_RUN ...]]

Positional arguments:
    
    TEST_TO_RUN           List tests to run (omit to run the default set, or use "all" to run all).

Optional arguments:

    -h, --help            show help message
    --config path/to/config.json
                            Specify the path to a JSON configuration file to extend the build configuration provided by the target
    --list, -l            List the tests that would be run, but don't run them. Implies --no-build
    --no-build, -n        Don't build first.
    -r, --release-build
    -d, --debug-build
    -G CMAKE_GENERATOR, --cmake-generator CMAKE_GENERATOR
                            CMake generator to use (defaults to Ninja). You can use this to generate IDE project files instead, see cmake --help for possible generator names
                                    
Run the tests for the current module on the current target. A build will be run first, and options to the build subcommand are also accepted by test.
This subcommand requires the target to provide a "test" script that will be used to run each test. Modules may also define a "testReporter" script, which will be piped the output from each test, and may produce a summary.


## Flashing a Project


    kubos start [-h] [--config path/to/config.json] [program]
    kubos flash [-h] [--config path/to/config.json] [program]
    
Positional arguments:

    program               name of the program to be started

Optional arguments:

    -h, --help            show help message
    --config path/to/config.json
                        Specify the path to a JSON configuration file to extend the build configuration provided by the target


Flashing your project using the kubos tool is a relatively straightforward process:

1. Ensure that your board is plugged into your computer

TODO: We probably want to add some info here on Virtualbox device filters and Guest Additions issues

2. Run the flash command

        $ kubos flash

*Note: If your current user does not have read/write permission to your hardware device you may need to run this command as root*

        $ sudo kubos flash
        
        
## Selecting a Target

    kubos target [-h] [-l] [set_target]

Positional arguments:
    
    set_target  set a new target board or display the current target

Optional arguments:
  
    -h, --help  show help message
    -l, --list  List all of the available target names

The Kubos SDK needs to know which target you intend to build for so it can select the proper cross compiler. Kubos currently supports several different targets:

| MCU Family   | Board  |
| ------------- |-------------|
| STM32F4 | STM32F407 Discovery Board |
|    |  STM32F405 PyBoard |
|  | STM32F405 NanoAvionics SatBus 3C0 OBC |
| MSP430     | MSP430F5529 Launchpad |
| ISIS       | ISIS-OBC |


The respective commands to select those targets are as follows.

        $ kubos target stm32f407-disco-gcc

        $ kubos target pyboard-gcc

        $ kubos target na-satbus-3c0-gcc

        $ kubos target msp430f5529-gcc
        
        $ kubos target kubos-linux-isis-gcc

To see all of the available targets run:

        $ kubos target --list
        
To see the currently set target run `kubos target` without any additional parameters.

## Updating Kubos

    kubos update [-h] [set_version]
    kubos use [-h] (-b BRANCH | set_version)
    
Positional arguments:

    set_version  Specify a version of the kubos source to use.

Optional arguments:

    -h, --help   show help message
    -b BRANCH, --branch BRANCH
                        Set the branch flag to specify to checkout a branch, not a tag
    
The Kubos is under constant development.  It's possible, and quite likely, that at some point a vagrant image might contain an outdated version of the Kubos repos.

In order to upgrade to the latest version, you can use the `kubos update` command.  If, instead, you'd like to downgrade to an older version, you can use the `set_version` positional parameter.

For example:

    $ kubos update v0.0.1
    
Use the `kubos use` command upgrade to an experimental version of Kubos.  For example:

    $ kubos use test-branch
    
This command will load the test-branch branch from the kubos repo
    
Use the `kubos version` command to see which version you're currently using and the `kubos versions` command to see what versions are available.

## Checking the Kubos Version

    kubos version [-h] [-l]

Optional arguments:
    
    -h, --help  show help message
    -l, --list  List all of the locally available KubOS source versions

Displays the current active version of the Kubos CLI and Kubos source repo.

## Listing Available Kubos Versions

    kubos versions [-h]

Optional arguments:
    
    -h, --help  show help message
    
Display the available versions of the KubOS source.



        

