Upgrading Your Kubos Development Environment
============================================

New updates will be announced to the Kubos community and the new releases will
be published through GitHub.
If there are any special steps required to upgrade to the newest release, they will be listed in the
release details.

Upgrading the Vagrant Box
-------------------------

Occasionally there will be an update or addition to one of the
components of the Vagrant environment. When any of these
components is changes we will package and release a new version of the
`kubos-dev` box.

If there's a new version of the box available you will see something
similar to the following when you start the box:

::

        $ vagrant up
        ...
        ==> default: A newer version of the box 'kubos/kubos-dev' is available! You currently
        ==> default: have version '0.2.2'. The latest is version '0.2.3'. Run
        ==> default: `vagrant box update` to update.
        ...

To manually check if your box is up-to-date you can run:

::

        $ vagrant box outdated

.. warning::  Updating your box will overwrite the filesystem inside the environment.

Moving projects into synced folders prevents them from being overwritten
since their actual location is external to the box. For more information
on setting up synced folders see the :ref:`mount-directory` section.

To update the box run:

::

        $ vagrant box update
        $ vagrant destroy
        $ vagrant up

.. note:: 

    ``vagrant destroy`` must be used if you want your current box to be
    updated to use the new version.
