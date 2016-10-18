## Base yotta Target Description for Compiling with any gcc cross compiler

This was froked from the [mbed-gcc](http://github.com/ARMmbed/target-mbed-gcc)
mbedOS target, and changed to be more modular and allow non-ARM cross compilers

You should not select this target to compile with directly (compilation will
probably not succeed without target-specific startup code).

See [CHANGELOG.md](CHANGELOG.md) for the changes associated with
each version.

## Code Coverage
To enable code coverage for a specific module, add this config to the application's config.json:

```JSON
    "debug" : {
        "options" : {
            "coverage" : {
                "modules" : {
                    "<module name>" : true
                }
            }
        }
    }
```

For example, to add code coverage to the sockets module, use this config:

```JSON
    "debug" : {
        "options" : {
            "coverage" : {
                "modules" : {
                    "sockets" : true
                }
            }
        }
    }
```

If building tests, then this config can be passed on the command line via the ```--config``` option. For example,

```
yotta build --config testconfig.json
```

```
yotta build --config '"debug" : { "options" : { "coverage" : { "modules" : { "sockets" : true } } } }'
```

## Configuring floating point support in `printf`

Floating point support in `printf` is enabled by default by this target
description. (Although derived targets may override this behaviour through the
[yotta config](http://yottadocs.mbed.com/reference/config.html) system.)

If you need to change the default behaviour (perhaps because the increase in
code-size is unacceptable to you), then you can do this in the `config` section
of a target description, or in an application's `config.json` file:

```JSON
  "gcc": {
    "printf-float": false
  }
```

Or:

```JSON
  "gcc": {
    "printf-float": true
  }
```
