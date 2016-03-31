## yotta Target Description for compiling natively on Linux

This is a [yotta target description](http://yottadocs.mbed.com/tutorial/targets.html).

Use this target to build things using the native compiler and runtimes (i.e.
running yotta and the compiler on the device you're compiling for).

This target can be used as the base target for more specific target
descriptions for different flavours of linux, but it will also work on its own
for simple programs.

```bash
yotta target linux-native
...
yotta install
```

