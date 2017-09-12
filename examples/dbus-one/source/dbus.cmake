# Two dbus includes
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -I/usr/include/dbus-1.0 -I/usr/lib/x86_64-linux-gnu/dbus-1.0/include")

# Link in libdbus-1
target_link_libraries(hello-nnmsg dbus-1)