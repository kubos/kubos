Kubos File Transfer Client
==========================

This client program can be used to test communication with the Kubos file transfer service.

It can be used to both send and receive files to/from the service.

Running the Client
------------------

To build and run the client program, run the following command from this folder::

    cargo run -- [config options] (upload|download) source-file [target-file] 
    
Required arguments:

    - Operation to perform

        - ``upload`` - Transfer ``source-file`` on the local host to ``target-file`` location
                       on the remote target
        - ``download`` - Transfer ``source-file`` on the remote target to ``target-file`` location
                       on the local host
    - ``source-file`` - The file to be transferred. May be a relative or absolute path.

Optional arguments:

    - ``target-file`` - Final destination path for the transferred file.
                        If not specified, the root file name from ``source-file`` will be used
                        and the file will be placed in the current directory of the destination.
    - ``-h {host IP}`` - Default: `0.0.0.0`. IP address of the local host to use.
    - ``-r {remote IP}`` - Default: `0.0.0.0`. IP address of the file transfer service to connect to.
    - ``-p {remote port}`` - Default: `8040`. UDP port of the file transfer service to connect to.
    - ``-P {host_port}`` - Default: `8080`. The UDP port that the file transfer service will send responses to.