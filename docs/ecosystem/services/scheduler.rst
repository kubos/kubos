Scheduler
=========

The KubOS system includes a scheduler service to facilitate recurring
and one time tasks with specific timing requirements.

Behavior
--------

The behavior of the scheduler is defined through three levels of organization: *config*
, *mode*, and *schedule*. A *config* is a file containing a list of tasks and the
relevant information for scheduling and executing the task. A *mode* is a directory which
contains many *config* files. The contents of all the *configs* found in a *mode*
make up the *schedule* of that mode.

Upon boot, or service start, the scheduler reads all config files in the active 
mode directory and schedules all tasks from that mode. The default active mode directory
is found at ``/home/system/etc/schedules/active``, which is a symlink
to the actual active mode's directory.

Schedules consist of three types of tasks: ``init``, ``one-time``, and
``recurring``. Tasks are loaded into the scheduler when the scheduler service starts,
when a mode becomes active, or when a new schedule file is imported into the active mode.
Any ``init`` tasks will be scheduled for execution when their corresponding
delay has passed. All other ``one-time`` or ``recurring`` tasks will be scheduled to
run at their designated times.

By default the scheduler comes with a single mode: ``safe``. This mode is reserved as a
fallback and default operating mode. Additional modes can be created and made active
through the GraphQL interface. All modes are represented as different directories in
the schedules directory (``/home/system/etc/schedules``).
Only one mode can be active at any given time.

The scheduler has its own log file, ``/var/log/kubos-schedule.log``, which
logs all schedule related actions the scheduler takes.

Schedule Specification
----------------------

Schedules are specified through config files in the `json` format. Each schedule config contains
all of the necessary information for each task to be scheduled. Multiple schedule configs can
coexist in the same schedule folder, allowing for easy loading of new scheduled tasks.

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
<<<<<<< HEAD
        "name": "Descriptive task-name",
        "delay": "Required start delay in Xh Ym Zs format"
        "app": {
            "name": "Required registered name of app to run",
            "args": ["Optional", "command", "line", "app", "args"],
            "config": "Optional path to app config",
=======
        "task-name": {
            "delay": "Required start delay in HH:mm:ss format"
            "app": {
                "name": "Required registered name of app to run",
                "args": ["Optional", "command", "line", "app", "args"],
                "config": "Optional path to app config",
            }
>>>>>>> Adding parsing schedule configs from files and init tasks.
        }
    }

Tasks in the ``one-time`` section will be executed once at a set time. Each task
in this section will be specified like so:

.. code-block:: json

    {
<<<<<<< HEAD
        "name": "Descriptive task-name",
        "time": "Required time of execution in yyyy-mm-dd hh:mm:ss format",
        "app": {
            "name": "Required registered name of app to run",
            "args": ["Optional", "command", "line", "app", "args"],
            "config": "Optional path to app config"
=======
        "task-name": {
            "time": "Required time of execution in yyyy-mm-dd hh:mm:ss format",
            "app": {
                "name": "Required registered name of app to run",
                "args": ["Optional", "command", "line", "app", "args"],
                "config": "Optional path to app config"
            }
>>>>>>> Adding parsing schedule configs from files and init tasks.
        }
    }

Tasks in the ``recurring`` section will be executed on a recurring basis. The task
will occur at the given ``period`` beginning after the ``delay`` has expired.
Each task in this section will be specified like so:

.. code-block:: json

    {
<<<<<<< HEAD
        "name": "Descriptive task-name",
        "delay": "Required start delay in Xh Ym Zs format",
        "period": "Required period of execution in Xh Ym Zs format",
        "app": {
            "name": "Required registered name of app to run",
            "args": ["Optional", "command", "line", "app", "args"],
            "config": "Optional path to app config"
=======
        "task-name": {
            "delay": "Required start delay in HH:mm:ss format",
            "period": "Required period of execution in HH:mm:ss format",
            "app": {
                "name": "Required registered name of app to run",
                "args": ["Optional", "command", "line", "app", "args"],
                "config": "Optional path to app config"
            }
>>>>>>> Adding parsing schedule configs from files and init tasks.
        }
    }

An example schedule config:

.. code-block:: json

    {
<<<<<<< HEAD
        "init": [
            {
                "name": "start-camera",
                "delay": "10m",
=======
        "init": {
            "start-camera": {
                "delay": "00:10:00",
>>>>>>> Adding parsing schedule configs from files and init tasks.
                "app": {
                    "name": "activate-camera"
                }
            }
        ],
        "one-time": [
            {
                "name": "deploy-solar",
                "time": "2019-08-11 15:20:10",
                "app": {
                    "name": "deploy-solar-panels"
<<<<<<< HEAD
                }
            }
        ],
        "recurring": [
            {
                "name": "clean-logs-every-12hrs":
                "delay": "1h",
                "period": "12h",
=======
                 }
            }
        },
        "recurring": {
            "clean-logs-every-12hrs": {
                "delay": "1:00:00",
                "period": "12:00:00",
>>>>>>> Adding parsing schedule configs from files and init tasks.
                "app": {
                    "name": "clean-logs"
                }
            }
        ]
    }

Service Configuration
---------------------

The scheduler service has the following available configuration parameter which may be
specified in the ``config.toml`` file under ``[scheduler-service]``.

- ``schedules-dir`` - (Default: ``/home/system/etc/schedules/``) The path to the
directory where modes and their schedules will be stored. This directory will be
created if it does not already exist.

The scheduler service also has the standard GraphQL interface parameters available for
configuration under ``[scheduler-service.addr]``.

- ``ip`` - The IP address of the GraphQL server
- ``port`` - The port the GraphQL server will listen on

GraphQL API
-----------

Queries
~~~~~~~

<<<<<<< HEAD
The scheduler exposes two queries, ``activeMode`` and ``availableModes``.
=======
The scheduler exposes a two queries, ``activeSchedule`` and ``availableSchedules``.
>>>>>>> Adding parsing schedule configs from files and init tasks.

The ``activeMode`` query  exposes information about the currently active
mode. It has the following schema::

    {
        activeMode: {
            name: String,
<<<<<<< HEAD
            path: String,
            lastRevised: String,
            schedules: [ScheduleConfigFile],
=======
            timeImported: String,
>>>>>>> Adding parsing schedule configs from files and init tasks.
            active: Boolean
        }
    }

<<<<<<< HEAD
The ``availableModes`` query  exposes information about the currently available
modes. It has the following schema::

    {
        availableModes(name: String): [
=======
The ``availableSchedules`` query  exposes information about the currently available
schedules. It has the following schema::

    {
        availableSchedules(name: String): [
>>>>>>> Adding parsing schedule configs from files and init tasks.
            {
               name: String,
<<<<<<< HEAD
               path: String,
               lastRevised: String,
               schedules: [ScheduleConfigFile],
=======
               timeImported: String,
>>>>>>> Adding parsing schedule configs from files and init tasks.
               active: Boolean
            }
        ]
    }

The ``ScheduleConfigFile`` object exposes metadata about individual schedule config files. It
has the following schema::

    {
        ScheduleConfigFile:
        {
            config: ScheduleConfig,
            path: String,
            name: String,
            timeImported: String
        }
    }

The ``ScheduleConfig`` object, and it's sub-objects, expose information about
individual schedule configs. They have the following schemas::

    {
        ScheduleConfig:
        {
            init: [ScheduleTask],
            oneTime: [ScheduleTask],
            recurring: [ScheduleTask]
        }

        ScheduleTask:
        {
            name: String,
            delay: String,
            app: ScheduleApp
        }

        ScheduleApp:
        {
            name: String,
            args: [String],
            config: String,
            runLevel: String
        }
    }


Mutations
~~~~~~~~~

<<<<<<< HEAD
The scheduler also exposes the following mutations: ``createMode``, ``removeMode``,
``activateMode``, ``importConfig``, and ``removeConfig``.
=======
The scheduler has two mutations: ``activate`` and ``import``.
>>>>>>> Adding parsing schedule configs from files and init tasks.

The ``createMode`` mutation instructs the scheduler to create a new empty schedule mode.
It has the following schema::

    mutation {
        createMode(name: String!) {
            success: Boolean,
            errors: String
        }
    }

<<<<<<< HEAD
The ``removeMode`` mutation instructs the scheduler to delete an existing mode's
directory and all schedules within. It cannot be applied to the currently active
mode, or to the *safe* mode. It has the following schema::

    mutation {
        removeMode(name: String!) {
            success: Boolean,
            errors: String
=======
The ``import`` mutation allows the scheduler to import a new schedule file and
make it available for use. It has the following schema::

    mutation {
        import(path: String!, name:String!): {
            success: Boolean!
>>>>>>> Adding parsing schedule configs from files and init tasks.
        }
    }

The ``activateMode`` mutation instructs the scheduler to make the specified mode
active. It has the following schema::

    mutation {
        activateMode(name: String!): {
            success: Boolean,
            errors: String
        }
    }

The ``safeMode`` mutation instructs the scheduler to make the *safe* mode
active. It has the following schema::

    mutation {
        safeMode(name: String!): {
            success: Boolean,
            errors: String
        }
    }

The ``importConfig`` mutation allows the scheduler to import a new schedule config into
a specified mode. If the targeted mode is active, all tasks in the config will be
immediately loaded for scheduling. It has the following schema::

    mutation {
        importConfig(path: String!, name: String!, mode:String!): {
            success: Boolean,
            errors: String
        }
    }

The ``removeConfig`` mutation allows the scheduler to remove a schedule config from
a specified mode. If the mode is active, all tasks in the config will be removed
from the scheduler. It as the following schema::

    mutation {
        removeConfig(name: String!, mode:String!): {
            success: Boolean,
            errors: String
        }
    }