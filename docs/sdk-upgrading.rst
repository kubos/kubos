Upgrading Your Kubos Development Environment
============================================

New updates will be announced on the `Kubos
website <http://docs.kubos.co/>`__. There will be instructions depending
on the specific release details of all the steps needed to upgrade for
the newest release.

Upgrading Kubos CLI
-------------------

Inside of a vagrant box lives the Kubos CLI. Upon new Kubos releases
there may be updates for the Kubos CLI.

First SSH into your kubos-dev box:

::

       $ cd <The path containing your kubos-dev Vagrantfile>
       $ vagrant up
       $ vagrant ssh

Then, upgrade the Kubos CLI:

::

        $ kubos update --cli

New releases of the Kubos CLI may add or remove specific commands or command options.
Tab completion is a feature that allows partially typed commands to be fully completed.
Tab completion can also suggest possible arguments by pressing the <tab> key while
typing a command. To keep tab completion synchronized with the Kubos CLI,
run the following command:

::

        $ kubos update --tab-completion

Upgrading the Kubos Source Modules
----------------------------------

To update the Kubos source modules your project will be built with run
the following command:

::

        $ kubos update

To check which version of the CLI you're using, use ``kubos version``

To list all of the available versions of the Kubos source modules run:

::

        $ kubos versions

To activate and use a new version of the Kubos source run:

::

        $ kubos use <version number>

The ``use`` command will checkout and replace the existing Kubos source
modules.

After running the ``use`` command, modules from the new version will be
linked. If the new release holds new modules or has removed existing
modules it may be necessary to re-link all modules to get rid of any
module conflicts between new and old versions.

To relink all of the Kubos source modules simply run:

::

        $ kubos link --all

Downgrading the Kubos Source Modules
------------------------------------

In the event you want to downgrade to an older version of the Kubos
source modules simply use the ``kubos use <version>`` command with the
older version number you want to downgrade to.

Upgrading Kubos-dev Vagrant Box
-------------------------------

Occasionally there will be an update or addition to one of the
components of the kubos-dev Vagrant environment. When any of these
components is changes we will package and release a new version of the
kubos-dev box.

If there's a new version of the box available you will see something
similar to the following when you start the box:

::

        $ vagrant up
        ...
        ==> default: A newer version of the box 'kubostech/kubos-dev' is available! You currently
        ==> default: have version '0.2.2'. The latest is version '0.2.3'. Run
        ==> default: `vagrant box update` to update.
        ...

To manually check if your box is up-to-date you can run:

::

        $ vagrant box outdated

Beware - Updating your box will overwrite the filesystem inside the environment.
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Moving projects into synced folders prevents them from being overwritten
since their actual location is external to the box. For more information
on setting up synced folders see the :ref:`mount-directory` section.

To update the box run:

::

        $ vagrant box update
        $ vagrant destroy
        $ vagrant up

Unfortunately the ``vagrant box update`` only downloads the changes but
does not apply them to your local environment. It's required to destroy
and re-create a new instance of the kubos-dev box.
