Scheduler
=========

The KubOS system includes a scheduler service to facilitate recurring tasks
and one time tasks with specific timing requirements.

Behavior
--------

Upon boot, or service start, the scheduler will read the active schedule file and
load the schedule into memory. The default active schedule file will be found at
``/home/system/etc/schedules/active.json``. Any ``init`` tasks will be performed
immediately. All other ``one-time`` or ``recurring`` tasks will be scheduled
to run at their designated times.

By default the scheduler will have two schedules: ``operational`` and ``safemode``. These
schedules are represented by different schedule files maintained in the schedules directory.
Only one schedule can be active at any given time.

The scheduler will have its own log file, ``/var/log/kubos-schedule.log``, which
will log all schedule related actions the scheduler takes.

Schedule Specification
----------------------

Schedules will be specified in files in the `json` format. Each schedule file will contain
all of the necessary information for that specific schedule. Different schedules, such as
operational, safe mode, etc, will each have their own schedule files.

Schedules consist of three sections: ``init``, ``one-time``, and ``recurring``. Each section
represents a different type of scheduled task. Each specified task in a section
represents the future scheduled execution of an app by the app service.

Inside of each section is the description of a task to schedule. Each task has an
associated ``app``. The scheduler currently delegates the actual running of tasks
to the ``app-service``, so each ``app`` contains the necessary information needed
by the ``app-service`` to run the app.

.. code-block::json

   {
       "app": {
           "name": "Required name of app as known by the app service",
           "args": ["Optional", "command", "line", "app", "args"],
           "config": "Optional path to app config file",
       }
   }

Tasks in the ``init`` section will be executed on boot or on schedule change. Tasks will be
assigned to the scheduler in an unpredictable order. The actual execution time
of the task will be affected by the associated delay times. If more than
one init task has the exact same delay, the execution order might be unpredictable.
Each task in this section will be specified like so:

.. code-block:: json

    {
        "task-name": {
            "delay": "Required start delay in HH:mm:ss format"
            "app": {
                "name": "Required registered name of app to run",
                "args": ["Optional", "command", "line", "app", "args"],
                "config": "Optional path to app config",
            }
        }
    }

Tasks in the ``one-time`` section will be executed once at a set time. Each task
in this section will be specified like so:

.. code-block:: json

    {
        "task-name": {
            "time": "Required time of execution in yyyy-mm-dd hh:mm:ss format",
            "app": {
                "name": "Required registered name of app to run",
                "args": ["Optional", "command", "line", "app", "args"],
                "config": "Optional path to app config"
            }
        }
    }

Tasks in the ``recurring`` section will be executed on a recurring basis. The task
will occur at the given ``period`` beginning after the ``delay`` has expired.
Each task in this section will be specified like so:

.. code-block:: json

    {
        "task-name": {
            "delay": "Required start delay in HH:mm:ss format",
            "period": "Required period of execution in HH:mm:ss format",
            "app": {
                "name": "Required registered name of app to run",
                "args": ["Optional", "command", "line", "app", "args"],
                "config": "Optional path to app config"
            }
        }
    }

An example schedule file:

.. code-block:: json

    {
        "init": {
            "start-camera": {
                "delay": "00:10:00",
                "app": {
                    "name": "activate-camera"
                }
            }
        },
        "one-time": {
            "deploy-solar": {
                "time": "2019-08-11 15:20:10",
                "app": {
                    "name": "deploy-solar-panels"
                 }
            }
        },
        "recurring": {
            "clean-logs-every-12hrs": {
                "delay": "1:00:00",
                "period": "12:00:00",
                "app": {
                    "name": "clean-logs"
                }
            }
        }
    }

Configuration
-------------

The scheduler has the following available configuration parameter which may be
specified in the ``config.toml`` file under ``[scheduler-service]``.

- ``schedules-dir`` - (Default: ``/home/system/etc/schedules/``) The path to the
directory where schedules will be stored. This directory will be created if it does
not already exist.

The scheduler also has the standard GraphQL interface parameters available for
configuration under ``[scheduler-service.addr]``.

- ``ip`` - The IP address of the GraphQL server
- ``port`` - The port the GraphQL server will listen on

GraphQL API
-----------

Queries
~~~~~~~

The scheduler exposes a two queries, ``activeSchedule`` and ``availableSchedules``.

The ``activeSchedule`` query  exposes information about the currently active
schedule. It has the following schema::

    {
        activeSchedule: {
            contents: String,
            path: String,
            name: String,
            timeImported: String,
            active: Boolean
        }
    }

The ``availableSchedules`` query  exposes information about the currently available
schedules. It has the following schema::

    {
        availableSchedules(name: String): [
            {
               contents: String,
               path: String,
               name: String,
               timeImported: String,
               active: Boolean
            }
        ]
    }


Mutations
~~~~~~~~~

The scheduler has two mutations: ``activate`` and ``import``.

The ``activate`` mutation instructs the scheduler to make the specified schedule active.
It has the following schema::

    mutation {
        activate(name: String!): {
            success: Boolean!
        }
    }

The ``import`` mutation allows the scheduler to import a new schedule file and
make it available for use. It has the following schema::

    mutation {
        import(path: String!, name:String!): {
            success: Boolean!
        }
    }
