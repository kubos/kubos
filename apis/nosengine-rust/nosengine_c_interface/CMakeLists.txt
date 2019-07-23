# NOSEngine is distributed as a CMake module. This CMake file just gathers up all of the
# components of NOSEngine into one shared object, which can be linked to from Rust.

cmake_minimum_required(VERSION 3.5)
project(nosengine_c_interface C)

# CMake has no way of printing text to stdout, except for message(STATUS ...),
# Which adds two leading dashes.
# This function just prints to stdout with no leading dashes.
# Source: https://stackoverflow.com/questions/15842828/is-there-a-good-way-to-display-clean-text-on-stdout-in-cmake
function(CleanMessage)
    execute_process(COMMAND ${CMAKE_COMMAND} -E echo "${ARGN}")
endfunction()

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} /usr/cmake/modules)

find_package(NOSENGINE REQUIRED COMPONENTS common transport client server i2c spi uart)

# Iterate over all of the NOSEngine libraries, and print them out so that Cargo can find them.
foreach(lib ${NOSENGINE_LIBRARIES})
    # NOSENGINE_LIBRARIES is a list of absolute paths to shared objects.
    # Here, the filename is split from the path and extension
    get_filename_component(lib_filename "${lib}" NAME_WE)
    # Remove the "lib" at the beginning of the filename
    string(REPLACE "lib" "" lib_name ${lib_filename})
    # After the above, something like "/usr/lib/libnosengine.so" will become "nosengine"
    # Cargo expects the linked libraries to be printed to stdout in a specific format.
    # See: https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script
    CleanMessage("cargo:rustc-link-lib=${lib_name}")

    # The following isn't necessary, because NOSEngine is normally installed in a normal
    # location for shared libraries (/usr/lib), so they are found automatically.
    # get_filename_component(lib_dir "${lib}" DIRECTORY)
    # CleanMessage("cargo:rustc-link-search=${lib_dir}")
endforeach(lib)

# This script doesn't actually build anything; all it has to do is print out the libraries.
# However, the CMake crate expects an install target, so I just make this meaningless installation that does nothing.

file(WRITE __lib.c)
set(nosengine_src __lib.c)

add_library(nosengine_c_interface SHARED ${nosengine_src})
install(TARGETS nosengine_c_interface LIBRARY DESTINATION lib ARCHIVE DESTINATION lib)
