#User Applications on the ISIS iOBC

- [Reference Documents](#reference-documents)
- [Building a Project](#building-a-project)
- [Updating Credentials](#updating-credentials)
- [Flashing the Board](#flashing-the-board)
- [Troubleshooting](#troubleshooting)
- [Debug Console](#debug-console)
- [Manual File Transfer](#manual-file-transfer)
- [Example Program](#example-program)

## Reference Documents

###iOBC Documentation

The ISIS-OBC Quickstart Guide should have been packaged with the iOBC and is a useful document for learning what each of the hardware components are, how to 
connect them, and what drivers need to be installed to support them.

###KubOS Documentation

- [Installing the KubOS-SDK](docs/cli-installing.md) - Basics of setting up the Kubos-SDK environment
- [Creating your first project](docs/first-project.md) - Steps to create and build a Kubos-SDK project (Note: Written for a KubOS RT end-target)
- [SDK Command Reference](docs/cli-reference.md) - Overview of the common Kubos-SDK commands
- [Kubos Linux Overview](docs/Linux_Overview.md) - Overview of the KubOS Linux components
- [Kubos Linux on iOBC](docs/Linux_on_iOBC.md) - Steps to build and load KubOS Linux for the iOBC

##Building a Project

In order to build a project for the ISIS-OBC, you'll need to create a Kubos-SDK project for KubOS Linux, set the correct target, and then build it.

    $ kubos init -l newproj
    $ kubos target kubos-linux-isis-gcc
    $ kubos build
    
##Updating Credentials

Ideally, you should not be using the default root user password.  If you've changed it, you'll need to pass the new password to the Kubos flash utility
through the config.json file.  Update the config>system>password parameter with the password to use.

    {
        "system" : {
            "password" : "newpass"
        }
    }

##Flashing the Board

The iOBC should be shipped with an FTDI cable.  This cable should be connected to the programming adapter, which should then be connected to the iOBC, to create the
debug UART connection.  User file transfer will take place using this connection.

** Note ** The Kubos flashing utility was configured with the assumption that an FTDI cable would be used.  If you have a different USB-to-serial cable type, you'll
either need to manually transfer the files, or contact me at catherine@kubos.co to request support for the new cable.

Assuming you've successfully built a Kubos-SDK project for the ISIS-OBC board, when you issue the 'kubos flash' the output should look like this:

    info: found newproj at source/newproj
    Compatible FTDI device found
    Sending file to board...
    /
    Transfer Successful
    Execution time: 21 seconds

##Troubleshooting

"No compatible FTDI device found"

    - Check that the iOBC is turned on and connected to your computer
    - Check that no other vagrant images are running.  Only one VM can have control of the USB, so it may be that another instance
    currently has control of the device.  You can shutdown a vagrant image with the command 'vagrant halt'
    - Verify that the USB is showing up within the vagrant environment with the 'lsusb' command.  You should see an FTDI device
    - Verify that the USB has been mapped to a linux device.  Issue the command 'ls /dev'.  You should see a /dev/ttyUSB* device. 
    If you don't, try rebooting your vagrant image ('vagrant halt', 'vagrant up')
    
"Transfer Failed: Connection Failed"

    - The SDK was unable to connect to the iOBC
    - Verify that the USB has been mapped to a linux device.  Issue the command 'ls /dev'.  You should see a /dev/ttyUSB* device. 
    If you don't, try rebooting your vagrant image ('vagrant halt', 'vagrant up')
    - If this error occurs after the transfer process has started, then the SDK likely lost connection to the iOBC.  Verify that
    the board is still correctly connected and powered and try the flash command again.

"Transfer Failed: Invalid Password"

    - The SDK was unable to log into the iOBC.  Verify that the password is correctly defined in your config.json file by issuing
    the 'kubos config' command.
    
System appears to have hung

    - If for some reason file transfer fails, it can take a couple minutes for the connection to time out and return control.
    - If you've waited a couple minutes and the system still appears hung, please let us know so that we can open a bug report.


##Debug Console

If the iOBC is correctly connected to your host computer, you should see a /dev/ttyUSB* device in your vagrant VM.  The VM is set up to automatically forward any
FTDI cables that connect to a /dev/FTDI device for ease-of-use.

The vagrant image comes pre-packaged with a minicom serial connection configuration file for the iOBC debug uart port.  You can connect with this configuration file
using the command

    $ minicom kubos
    
Alternatively, you can manually create a serial connection with minicom (or other terminal application) using the following connection configuration:

| Setting   | Value  |
|-----------|--------|
| Baudrate  | 115200 |
| Bits      | 8      |
| Parity    | N      |
| Stop Bits | 1      |

Once connected, you can log in using either a user that you've created, or root, which has a default password of 'Kubos123'.

Fully logged in, the console should look like this:

    Welcome to KubOS Linux
    
    (none) login: root
    Password: 
    Jan  1 00:00:16 login[212]: root login on 'ttyS0'
    ~ # 

##Manual File Transfer

If for some reason you want to manual transfer a specific file onto the iOBC, for example a custom script, you'll need to do the following:

Connect to the board through minicom (the file transfer protocol is not guaranteed to work with any other terminal program)

    $ minicom kubos
    
Login to the board

    Welcome to minicom 2.7
    
    OPTIONS: I18n
    Compiled on Feb  7 2016, 13:37:27.
    Port /dev/FTDI, 21:26:43
    
    Press CTRL-A Z for help on special keys
    
    
    Welcome to KubOS Linux
    (none) login: root
    Password:
    Jan  1 00:00:11 login[210]: root login on 'ttyS0'
    ~ #


Navigate to the location you'd like the received file to go.

    ~ # mkdir righthere
    ~ # cd righthere
    ~/righthere #


Issue the zModem command to prep the board to receive a file

    $ rz -bZ
    
Press **Ctrl+a**, then press **s** to open the minicom file transfer dialog.

    +-[Upload]--+
    | zmodem    |
    | ymodem    |
    | xmodem    |
    | kermit    |
    | ascii     |
    +-----------+

Select zmodem

    +-------------------[Select one or more files for upload]-------------------+
    |Directory: /home/vagrant                                                   |
    | [..]                                                                      |
    | [linux]                                                                   |
    | [newprj]                                                                  |
    | minicom.log                                                               |
    |                                                                           |
    |              ( Escape to exit, Space to tag )                             |
    +---------------------------------------------------------------------------+
    
                   [Goto]  [Prev]  [Show]   [Tag]  [Untag] [Okay]

Select the file to send (Press 'g' to open the Goto dialog and Enter to open the file selector dialog)

    +-------------------[Select one or more files for upload]-------------------+
    |Directory: /home/vagrant/linux/build/kubos-linux-isis-gcc/source           |
    | [..]                                                                      |
    | [CMakeFiles]                                                              |
    | CMakeLists.txt                                                            |
    | CTestTestfile.cmake                                                       |
    | cmake_install.cmake                                                       |
    | linux                                                                     |
    | linux.map                                                                 |
    |                +-----------------------------------------+                |
    |                |No file selected - enter filename:       |                |
    |                |> linux                                  |                |
    |                +-----------------------------------------+                |
    |                                                                           |
    |              ( Escape to exit, Space to tag )                             |
    +---------------------------------------------------------------------------+
    
                   [Goto]  [Prev]  [Show]   [Tag]  [Untag] [Okay]

You should see a progress dialog as your file is transferred to the board.

    +-----------[zmodem upload - Press CTRL-C to quit]------------+
    |^XB00000000000000rz waiting to receive.Sending: linux        |
    |Bytes Sent:  41984/  99084   BPS:8905     ETA 00:06          |
    |                                                             |
    |                                                             |
    |                                                             |
    |                                                             |
    |                                                             |
    +-------------------------------------------------------------+
    
Once file transfer is complete, you should be able to press enter and use your new file

    +-----------[zmodem upload - Press CTRL-C to quit]------------+
    |^XB00000000000000rz waiting to receive.Sending: linux        |
    |Bytes Sent:  99084   BPS:7982                                |
    |                                                             |
    |Transfer complete                                            |
    |                                                             |
    | READY: press any key to continue...                         |
    |                                                             |
    +-------------------------------------------------------------+
    
Press **Ctrl+a**, then **q** to bring up the dialog to exit minicom.  Hit enter to quit without reset.


##Example Program

Let's walk through the steps to create the example KubOS Linux project.

Initialize the project

    $ kubos init --linux newproj
    
Move into the project folder

    $ cd newproj
    
Set the project target

    $ kubos target kubos-linux-isis-gcc
    
Build the project

    $ kubos build
    
Flash the project

    $ kubos flash
    
Log into the board

    $ minicom kubos
    Login: root/Kubos123

Run the example application

    $ newproj
    
Output should look like this:

    Initializing CSP
    Starting example tasks
    Ping result 80 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 90 [ms]
    Packet received on MY_PORT: Hello World
    Ping result -1 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 60 [ms]
    Packet received on MY_PORT: Hello World
    Ping result 50 [ms]
    Packet received on MY_PORT: Hello World
    
Press **Ctrl+C** to exit execution.
