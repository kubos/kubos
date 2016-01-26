## Base yotta Target Description for Compiling with arm-none-eabi-gcc

This is a base [yotta target
description](http://docs.yottabuild.org/tutorial/targets.html) for compiling
mbed OS with the arm gcc cross-compiler. Other target descriptions can inherit
from it and add or override things as necessary (such as the link script, or
preprocessor definitions).

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
