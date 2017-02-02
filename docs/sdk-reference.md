# Kubos CLI Command Reference

The `kubos` command is always run with a subcommand in order to do something, `kubos` with no subcommand will only display help information.

## Command Overview

    [build](#kubos-build)               Build the current module.
    [clean](#kubos-clean)              Remove files created by kubos and the build.
    [config](#kubos-config)              Display the target configuration info.
    [debug](#kubos-debug)               Attach a debugger to the current target.  Requires target support.
    [init](#kubos-init)                Create a new module.
    [licenses](#kubos-licenses)            List the licenses of the current module and its dependencies.
    [link](#kubos-link)                Symlink a module
    [link-target](#kubos-link-target)         Symlink a target
    [list](#kubos-list)                List the dependencies of the current module, or the inherited targets of the current target.
    [outdated](#kubos-outdated)            Display information about dependencies which have newer versions available.
    [remove](#kubos-remove)              Remove or unlink a dependency without removing it from module.json.
    [shrinkwrap](#kubos-shrinkwrap)          Create a yotta-shrinkwrap.json file to freeze dependency versions.
    [start](#kubos-start)               Launch the compiled program (available for executable modules only). Requires target support for cross-compiling targets.
    [target](#kubos-target)              Set or display the target device.
    [test](#kubos-test)                Run the tests for the current module on the current target. Requires target support for cross-compiling targets.
    [version](#kubos-version)             Display the current active version of the cli and KubOS source repo.
    [versions](#kubos-versions)            Display the available versions of the KubOS source.
    [update](#kubos-update)              Download newer versions of the KubOS Modules
    [use](#kubos-use)                 Set a new version of the KubOS modules to build your projects against.



### kubos build

        $ kubos build [--generate-only] [--debug-build] [--cmake-generator <cmake-generator-name>] [name ... ]
        $ kubos build [ ... ] -- [ build-tool arguments ]

#### Description

Build the current module and its dependencies.

Kubos uses CMake to control the build, the basic process is:

* kubos generates CMakeLists.txt describing the libraries and executables to build
* kubos instructs CMake to generate the make files / ninja files / IDE project file (depending on --cmake-generator)
* kubos instructs CMake to execute the build. The compiler used depends on the CMake Toolchain file provided by the active kubos target.

#### Options

* `--generate-only`, `-g`: only generate the CMakeLists, don't build

* `--debug-build`, `-d`: build a debug (less-optimised) build.
The effects depend on the target (this selects CMake build type Debug), but generally this means no optimisation, and NDEBUG is not defined.

* `--release-build`, `-r`: build a release (optimised) build. deprecated
The effects depend on the target (this selects CMake build type RelWithDebInfo). This option is deprecated because it is now the default, unless --debug-build is specified.

* `--cmake-generator`, `-G`: specify the CMake Generator. CMake can generate project files for various editors and IDEs.

* `name ...`: one or more modules may be specified, in which case only these modules and their dependencies will be built. Use `all_tests` to cause all tests to be built.

* `-- ...`: any options specified after -- are passed unmodified on to the tool being used for building (e.g. Ninja, or make)

### kubos init

#### Synopsis

        $ kubos init <project name> [--linux] [--rt]

#### Description

Create a new subdirectory containing a new project named the same as the argument provided. If a subdirectory already exists, the command will abort and not delete or overwrite any files.

#### Options

* `--linux`, `-l` Create the new project as a linux application for KubOS Linux
* `--rt`, `-r`    Create the new project as a KubOS RT project

### kubos test
#### Synopsis

        $ kubos test [--list] [--no-build] [ build-arguments ] [tests-to-run ...]

#### Description

Run tests. If no arguments are specified, then the tests for the current module
will be run, use `kubos test all` to run the tests for all modules.

The target description provides support for the test
command if it is a cross-compiling target (no support is necessary to run tests
natively). The `scripts.test` value in the target description is executed with
`$program` expanded to the path to the binary, it should be a wrapper script
that loads the binary at the specified path onto the target device, runs it,
and prints output on standard output.

Options:

 * `--list`, `-l`: List the tests that would be run, rather than running them.
   Implies `--no-build`.
 * `--no-build`, `-n`: Don't build anything, try to run already-built tests.
   Things will fail if all the specified tests are not built!
 * This command also accepts the options to `kubos_build`,
   which are used if building.

#### Examples

        $ kubos test
        $ kubos test --list all
        $ kubos test -n my-test
        $ kubos test --config="path/to/test-config.json"

# kubos debug
#### Synopsis


        $ kubos debug


#### Description

If the target description supports it, launch a debugger attached to the specified executable.


### kubos target
#### Synopsis

        $ kubos target
        $ kubos target <targetname>
        $ kubos target --list, -l

#### Description
Display or set the current target.

Targets define the options and commands that `kubos` uses to compile modules and executables.

A target must define a CMake Toolchain file describing all of the rules that `kubos` uses to build software, it may also define commands to launch a debugger (used by `kubos debug`).

#### Arguments
* `--list`, `-l` List all of the available KubOS targets.

#### Examples

        $ kubos target stm32f407-disco-gcc

### kubos update
#### Synopsis

        $ kubos update
        $ kubos update <version number>

#### Description
Pull and update all of the current KubOS modules. By default if no `<version number>`

#### Arguments
`<version number>` If a version number is specified then kubos will try to checkout the provided version number after pulling the latest updates


### kubos version
#### Synopsis

        $ kubos version

#### Description
Display the current version of the Kubos CLI, and the KubOS modules


### kubos link
Synonyms: `kubos ln`
#### Synopsis

        $ kubos link (in a module directory)
        $ kubos link <modulename>
        $ kubos link /path/to/a/module

#### Description
Module linking allows you to use local versions of modules when building other modules – it's useful when fixing a bug in a dependency that is most easily reproduced when that dependency is used by another module.

By default all of the KubOS modules are linked into all new projects.

To link a module there are two steps. First, in the directory of the dependency:

        $ kubos link

This will create a symlink from the global modules directory to the current module.

Then, in the module that you would like to use the linked version of the dependency, run:

        $ kubos link <depended-on-module-name>

When you run `kubos build` it will then pick up the linked module.

This works for direct and indirect dependencies: you can link to a module that your module does not use directly, but a dependency of your module does.

The variant of the command which takes a path to an existing module (e.g. `kubos link ../path/to/a/module`) performs both steps in sequence, for convenience.

#### Arguments

`--all`, `-a` Link all of the default KubOS modules and targets into a project in the current directory

#### Directories
When you run `kubos link`, links are created in a system-wide directory under
`yotta_PREFIX`, and the links in that directory are then picked up by
subsequent `kubos link <modulename>` commands.


###  kubos link-target
#### Synopsis

        $ kubos link-target (in a target directory)
        $ kubos link-target <targetename>
        $ kubos link-target /path/to/a/target

#### Description
Like module linking, target linking allows you to use local versions of targets when building modules – it's useful when developing and testing target descriptions.

By default all of the KubOS targets will be linked into all new projects.

To link a target you need to perform two steps. First, in the directory of the target:

        $ kubos link-target

This will create a symlink from the global targets directory to the current target.

Then, in the module that you would like to use the linked version of the target, run:

        $ kubos link-target <targetename>

When you run `kubos build` (provided you've set `kubos target` to `<targetname>`), the linked target description will be used.

The variant of the command which takes a path to an existing module (e.g. `kubos link ../path/to/a/module`) performs both steps in sequence, for convenience.

See also [kubos link](#kubos-link).


### kubos list
Synonyms: `kubos ls`
#### Synopsis

        $ kubos list [--all]
        $ kubos list [--json]

#### Description
List the installed dependencies of the current module, including information on
the installed versions. Unless `--all` is specified, dependencies are only
listed under the modules that first use them, with `--all` dependencies that
are used my multiple modules are listed multiple times (but all modules will
use the same installed instance of the dependency).

The `--json` option will cause the list to be output in JSON format, for
example:

```
{
  "modules": [
    {
      "name": "toplevel-module-name",
      "version": "1.0.0",
      "path": "/some/path/on/disk/toplevel-module-name",
      "specifications": [
        {
          "version": "~0.11.0",
          "name": "some-dependency-name"
        }
      ]
    },
    {
      "name": "some-dependency-name",
      "version": "0.11.7",
      "path": "/some/path/on/disk/yotta_modules/some-dependency-name",
      "linkedTo": "/some/path/on/disk/some-dependency-name",
      "specifications": [
        {
          "version": "ARMmbed/some-test-dependency#^1.2.3",
          "name": "some-test-dependency",
          "testOnly": true
        }
      ]
    },
    {
      "name": "some-test-dependency",
      "version": "1.5.6",
      "path": "/some/path/on/disk/yotta_modules/some-test-dependency",
      "errors": [
        "a description of some error with this module"
      ]
    }
}
```

###  kubos licenses
#### Synopsis


        $ kubos licenses [--all]


#### Description
List the licenses of all of the modules that the current module depends on. If
`--all` is specified, then each unique license is listed for each module it
occurs in, instead of just once.

**NOTE:** while kubos can list the licenses that modules have declared in their
`module.json` files, it can make no warranties about whether modules contain
code under other licenses that have not been declared.

### kubos config
#### Synopsis


        $ kubos config


#### Description
Display the merged config data for the current target
(and application, if the current module defines an executable application).

The config data is produced by merging the json config data defined by the
application, the current target, and any targets the current target inherits
from recursively. Values defined by the application will override those defined
at the same path by targets, and values defined in targets will override values
defined by targets they inherit from.

The config data displayed is identical to the data that will be available to
modules when they are built.

