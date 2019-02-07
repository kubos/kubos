Kubos SDK Cheatsheet
====================

.. note::

    This doc refers to the process used for creating and interacting with projects written in C.
    Please see the separate docs for details about using :doc:`Rust <sdk-rust>` or :doc:`Python <sdk-python>`.

Creating a Project
------------------

Create a folder for your project with the ``mkdir`` command followed by
the name of your project.

Inside of this folder you will create three new folders: one named after
your project, one named ``source`` and one named ``test``. This is the skeleton
folder structure of your C project.

You also need to create a text file called ``CMakeLists.txt``. This is where
the build instructions for your project will go.

Here is a sample ``CMakeLists.txt`` for a library project::


        cmake_minimum_required(VERSION 3.5)
        project(project-name VERSION 0.1.0)

        add_library(project-name
          source/lib.c
        )

        target_include_directories(project-name
          PUBLIC "${project-name_SOURCE_DIR}/project-name"
        )


Here is a sample ``CMakeLists.txt`` for an executable project::

        cmake_minimum_required(VERSION 3.5)
        project(project-name VERSION 0.1.0)

        add_executable(project-name
          source/main.c
        )


Here is a quick rundown of the files needed to start your project:

+-------------------+---------------------------------------------------------------------------+
| File/folder       | Description                                                               |
+===================+===========================================================================+
| `project-name`    | This folder is where header files live                                    |
+-------------------+---------------------------------------------------------------------------+
| `source`          | This folder is where source files live                                    |
+-------------------+---------------------------------------------------------------------------+
| `test`            | This folder is where test source files live                               |
+-------------------+---------------------------------------------------------------------------+
| `CMakeLists.txt`  | This file contains the CMake build configuration                          |
+-------------------+---------------------------------------------------------------------------+

Kubos uses the CMake build system, along with our own folder conventions, for C projects.
 You can read more about CMake `here https://cmake.org/cmake-tutorial/`__.

Building a Project
------------------

To build a Kubos C project, the ``CMake`` build mechanism needs to be invoked.

First a folder will be created to store the build artifacts::

        $ mkdir build
        $ cd build

Next ``CMake`` will be called and told where to find the build instructions::

        $ cmake ..

Lastly ``make`` will be called to execute the build setup by ``Cmake``::

        $ make

All build artifacts will be generated in the ``build`` folder.
If the project is an executable then the binary will be named ``project-name``.
If the project is a library then the library file will be named ``libproject-name.a``.

To pick up on any changes to the project`s CMake files run ``cmake ..``
again followed by ``make``.

To build a project from scratch run ``make clean`` to remove all prior
build artifacts followed by ``make``.

.. _cross-compiling:

Cross Compiling
---------------

CMake needs to know which target you intend to build for so it can
select the proper cross compiler. Kubos currently supports several
different targets:

+------------+-----------------------------------------------+---------------------------------------------------+
| Vendor     | Toolchain                                     | Description                                       |
+============+===============================================+===================================================+
| ISIS       | /usr/bin/iobc_toolchain/usr/bin/arm-linux-gcc | ISIS-OBC                                          |
+------------+-----------------------------------------------+---------------------------------------------------+
| Pumpkin    | /usr/bin/bbb_toolchain/usr/bin/arm-linux-gcc  | Pumpkin Motherboard Module 2                      |
+------------+-----------------------------------------------+---------------------------------------------------+
| Beaglebone | /usr/bin/bbb_toolchain/usr/bin/arm-linux-gcc  | Beaglebone Black, Rev. C                          |
+------------+-----------------------------------------------+---------------------------------------------------+
| (Vagrant)  | /usr/bin/gcc                                  | Native x86 Linux                                  |
+------------+-----------------------------------------------+---------------------------------------------------+

By default CMake will compile with the local ``gcc`` toolchain found on the PATH.
If you are working inside of the SDK VM then the native target is x86 Linux.

If you would like to cross-compile for one of the supported embedded boards then
CMake will need to be informed about which cross-compiling toolchain to use. CMake
looks at two environment variables when compiling to determine which toolchain it should use.
These variables are ``CC`` and ``CXX``.

For example

::

       $ mkdir build && cd build
       $ export CC=/usr/bin/bbb_toolchain/usr/bin/arm-linux-gcc
       $ export CXX=/usr/bin/bbb_toolchain/usr/bin/arm-linux-g++
       $ cmake .. && make


Flashing your Project
---------------------

Ensure that your board is plugged into your computer. 

Running the following command will list all of the available devices in your
Kubos SDK box.

   ::

       $ lsusb
