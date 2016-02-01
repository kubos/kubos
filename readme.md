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

