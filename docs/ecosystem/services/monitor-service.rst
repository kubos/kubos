Monitor Service
===============

The monitor service is a special hardware service which is included by default in KubOS.
Instead of having an external hardware endpoint, this service's endpoint is the OBC itself.

The monitor service provides a way to check currently running processes and total system memory
usage.

Interface Details
-----------------

Specific details about the available GraphQL queries can be found in the |monitor| Rust docs.

 .. |monitor| raw:: html

    <a href="../../rust-docs/monitor_service/index.html" target="_blank">monitor service</a>

PS Query
--------

The ``ps`` query allows users to get the status of running processes on their OBC.

It has the following schema::

    {
        ps(pids: [Int!] = null): [
            {
                pid: Int!
                uid: Int
                gid: Int
                usr: String
                grp: String
                state: String
                ppid: Int
                mem: Int
                rss: Int
                threads: Int
                cmd: String
            }
        ]
    }

The ``pids`` input parameter specifies an array of process ID numbers which the service should fetch
information about. If the parameter is not specified, then information about all currently running
processes is returned.

For each requested process, the query can return the following data:

    - ``pid`` - The process ID of the process
    - ``uid`` - The user ID of the user or process which created this process
    - ``gid`` - The group ID of the user or process which created this process
    - ``usr`` - The user name associated with the UID
    - ``grp`` - The group name associated with the GID
    - ``state`` - A single character indicating the process' current state. Please refer to
      `this manual <http://man7.org/linux/man-pages/man1/ps.1.html#PROCESS_STATE_CODES>`__ for a
      description of each character's meaning
    - ``ppid`` - The process ID of the process which started this process
    - ``mem`` - The virtual memory size of the process, in bytes
    - ``rss`` - The current number of pages the process has in real memory
    - ``threads`` - The current number of threads in this process
    - ``cmd`` - The full command, including arguments, which was used to execute this process
      (taken from `/proc/{pid}/cmdline`. Defaults to the raw process name if the file cannot be read)

An example query might look like this::

    {
        ps(pids: [473, 477, 501]) {
            pid, 
            ppid, 
            threads, 
            cmd
        }
    }
        
The response from the service might look like this::

    {
        "errors":"",
        "data": {
            "ps":[
                {
                    "cmd": "/usr/sbin/file-service",
                    "pid": 473,
                    "ppid": 1,
                    "threads": 1
                },
                {
                    "cmd": "/usr/sbin/telemetry-service",
                    "pid": 477,
                    "ppid": 1,
                    "threads": 2
                },
                {
                    "cmd": "monitor-service",
                    "pid": 501,
                    "ppid": 497,
                    "threads": 1
                }
            ]
        }
    }

MemInfo Query
-------------

The ``memInfo`` query can be used to get information about the memory availablity and usage of the
system as a whole. It works by reading and parsing the `/proc/meminfo` file.

It has the following schema::

    {
        memInfo {
            total: Int
            free: Int
            available: Int
            lowFree: Int
        }
    }
    
The query has the following response fields:

    - ``total`` - The total usable RAM of the system, in kB
    - ``free`` - The total amount of free memory (includes lowFree)
    - ``available`` - An estimate of how much memory is available for starting new applications,
      without swapping
    - ``lowFree`` - The amount of free memory which can be used by the kernel

.. note::

    Not all response fields are available on all systems.
    They will be omitted from the response if they are not available.