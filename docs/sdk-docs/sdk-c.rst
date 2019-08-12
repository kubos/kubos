Using C with the Kubos SDK
==========================

Creating a Project
------------------

Create a folder for your project with the ``mkdir`` command followed by
the name of your project.

Inside of this folder you will create three new folders:

* `project-name` - This is where any header (`*.h`) files will go.
* `source` - This is where any source (`*.c`) files will go.
* `test` - This is where any test files will go.

This is the skeleton folder structure of your C project.

You also need to create a text file called ``CMakeLists.txt``. This is where
the build instructions for your project will go.

Here is a quick rundown of the files needed to start your project:

Your final directory structure should look like this::

    project-name
    |_ CMakeLists.txt
    |_ project-name
    |_ source
    |_ test

Kubos uses the CMake build system, along with our own folder conventions, for C projects.
You can read more about CMake `here <https://cmake.org/cmake-tutorial/>`__.

Working with CMakeLists.txt
---------------------------

The ``CMakeLists.txt`` file provides instructions to ``cmake`` on how
to build the ``C`` project.

All ``CMakeLists.txt`` files need to start with these two lines::

        cmake_minimum_required(VERSION 3.5)
        project(project-name VERSION 0.1.0)

Next, ``cmake`` must be told what type of project it is building
and which source files to compile.

If the project is a library, then the ``add_library`` command
will be used::

        add_library(project-name
          source/lib.c
        )

If the project is an executable, then the ``add_executable``
command will be used::

        add_executable(project-name
          source/main.c
        )

Lastly, the header folder needs to be added using the
``target_include_directories`` command::

        target_include_directories(project-name
          PUBLIC "${project-name_SOURCE_DIR}/project-name"
        )

Building a Project
------------------

To build a Kubos C project, the ``CMake`` build mechanism needs to be invoked.

First, a folder will be created to store the build artifacts::

        $ mkdir build
        $ cd build

Next, ``CMake`` will be called and told where to find the build instructions::

        $ cmake ..

Lastly, ``make`` will be called to execute the build setup by ``Cmake``::

        $ make

All build artifacts will be generated in the ``build`` folder.
If the project is an executable, then the binary will be named ``project-name``.
If the project is a library, then the library file will be named ``libproject-name.a``.

To build a project from scratch, run ``make clean`` to remove all prior
build artifacts followed by ``make``.

To pick up on any changes to the project`s CMake files, run ``cmake ..``
again, followed by ``make``.

To build a project without any of the prior ``make`` or ``cmake`` artifacts,
remove the ``build`` directory with all of its contents and start the build
process over.

.. _cross-compiling:

Cross Compiling
---------------


By default, CMake will compile with the local ``gcc`` toolchain found on the PATH.
If you are working inside of the SDK VM, then the native target is x86 Linux.

The SDK also provides cross-compiling toolchains for several different targets:

+------------+-----------------------------------------------+------------------------------+
| Vendor     | Toolchain                                     | Description                  |
+============+===============================================+==============================+
| ISIS       | /usr/bin/iobc_toolchain/usr/bin/arm-linux-gcc | ISIS-OBC                     |
+------------+-----------------------------------------------+------------------------------+
| Pumpkin    | /usr/bin/bbb_toolchain/usr/bin/arm-linux-gcc  | Pumpkin Motherboard Module 2 |
+------------+-----------------------------------------------+------------------------------+
| Beaglebone | /usr/bin/bbb_toolchain/usr/bin/arm-linux-gcc  | Beaglebone Black, Rev. C     |
+------------+-----------------------------------------------+------------------------------+
| (Vagrant)  | /usr/bin/gcc                                  | Native x86 Linux             |
+------------+-----------------------------------------------+------------------------------+

If you would like to cross-compile for one of the supported embedded boards, then
CMake will need to be informed about which cross-compiling toolchain to use. CMake
looks at two environment variables when compiling to determine which toolchain it should use.
These variables are ``CC`` and ``CXX``.

For example::

       $ mkdir build && cd build
       $ export CC=/usr/bin/bbb_toolchain/usr/bin/arm-linux-gcc
       $ export CXX=/usr/bin/bbb_toolchain/usr/bin/arm-linux-g++
       $ cmake .. && make

.. _c-transfer:

Transferring
------------

Executables generated from C projects can be transferred to the target OBC :ref:`via a supported file
transfer method <file-transfer>`.

Binaries may be transferred to any location on the target board, however, they should be copied
to `/home/system/usr/bin` if you would like them to be automatically accessible via the system PATH.

