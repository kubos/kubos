## 1.1.0
 * Add support for building individual modules with code coverage

## 1.0.0
 * Switch on C++11 compilation by default. This is technically a breaking
   change, but the mbed-OS modules themselves have been patched so that they
   continue to work correctly.

## 0.1.4
 * add config for mbed default baud rate

## 0.1.3
 * update for compatibility with CMake 3.4

## 0.1.2
 * set dwarf debug info version to 3, for compatibility with the uVision
   debugger.

## 0.1.1
 * add support for a `GLOBALLY_LINKED_TARGET_LIBS` variable that derived
   targets can set to specify libraries that should be linked into every
   executable.

## 0.1.0
 * add `<builddir>/generated/include` to the header saerch paths: this is the
   canonical place to put generated headers (prefixed by module name to avoid
   clashes)

## 0.0.14
For this and prior releases, no changes were tracked. Please see the git
history
