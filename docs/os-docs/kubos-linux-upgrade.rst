Upgrading Kubos Linux
=====================

Overview
--------

Kubos Linux upgrades are distributed as `kpack-{YYYY.MM.DD}.itb` files.
ITB stands for 'Image Tree Blob' and allows Kubos to utilize the
existing DFU utility currently available in U-Boot.

Within each file will be a new version of the kernel image and root
filesystem.

To upgrade a board currently running Kubos Linux, an upgrade package
will be loaded into the upgrade partition of the board. For now, this
can be done through the Kubos SDK or by manually copying the package
into the upgrade partition.

Once the board is rebooted, U-Boot will take the package and then
install each component into the appropriate partition (kernel/rootfs).
If installation completes successfully, then the board will be rebooted
and then boot into the new version of Kubos Linux.

The overall flow looks like this:

.. figure:: ../images/kubos_linux_upgrade.png
   :alt: Kubos Linux Upgrade

   Kubos Linux Upgrade

.. note::

    User files should not be impacted by upgrade installation as long as
    they remain under the /home directory. This directory maps to the
    user space partition.
    
    However, some future releases may cause the Kubos libraries to undergo
    significant changes. In this case, backwards compatilibity is not
    guaranteed and user applications may need to be rebuilt.

.. _upgrade-installation:

Upgrade Installation
--------------------

Pre-requisites
~~~~~~~~~~~~~~

The SD card should have been formatted with the correct partitions. If
not, refer to the :ref:`install-sd` instructions.

The host computer should be connected to the target board, which should
be on and running Kubos Linux.

A Kubos SDK Vagrant box should be installed on your host computer and at least
one shared folder should be set up. Installation instructions can be
found :doc:`here <../installation-docs/sdk-installing>`.

Installation
~~~~~~~~~~~~

Acquire an upgrade package from https://github.com/kubos/kubos-linux-build/releases.


.. note::

    Not all releases can be installed via upgrade. Some releases contain
    changes which massively alter the system, causing a full install to be
    required instead.


Transfer the package to the target system using scp::

   $ scp kpack-{version}.itb kubos@{target_ip}:/upgrade/

Refer to the :ref:`flash-troubleshooting` section if anything goes wrong
with the transfer.

Once the transfer has completed successfully, trigger a reboot of the
board. This can be done with the Linux ``reboot`` command. Once job
scheduling has been implemented, you will be able to schedule the
desired reboot time.

When the board boots into U-Boot, the new package will be detected and
loaded. If the loading is successful, the board will reboot into the
newly installed Kubos Linux. The U-Boot console messages will look
similar to this:

::

    Processing upgrade 'kernel@1' :crc32+ sha1+ 
    ###writing kernel
    1154936 bytes written
    Processing upgrade 'rootfs@1' :crc32+ sha1+ 
    ##########################Un-Protected 1 sectors
    Erasing Flash...
    . done
    Erased 1 sectors
    Writing to Flash... done
    Protected 1 sectors
    resetting ...
    reset_cpu
    
.. _upgrade-rollback:

Upgrade Rollback
----------------

If for some reason you need to rollback to a previous version of Kubos
Linux, you don't need to reflash the board with the correct upgrade
package. Previous packages are not deleted once they have been loaded.
As a result, you can simply specify which package you would like to boot
into and then restart your system.

From the Kubos Linux shell:

::

    $ fw_setenv kubos_updatefile kpack-{desired version}.itb
    $ reboot

.. _upgrade-creation:

Upgrade Creation
----------------

This section is for developers who have made changes to Kubos Linux and
want to generate an upgrade package.

Pre-requisite
~~~~~~~~~~~~~

Build the new OS. Refer to the :ref:`build-os` instructions.

Run the Packaging Script
~~~~~~~~~~~~~~~~~~~~~~~~

From the 'kubos-linux-build/tools' folder, run the ``kubos-package.sh``
script. This will create the `rootfs.img` and `kubos-kernel.itb` files and
then use the `kpack.its` file to bundle them into an `\*.itb` file. This is
the file that will be distributed to customers when an upgrade is
needed.

The automatically generated naming convention for the package is
kpack-*yyyy*-*mm*-*dd*.itb

Custom Packages
^^^^^^^^^^^^^^^

If you'd like to customize the package, there are a few different
options available through the script:

-  -t {target} : **Required** Specifies the name of the target board,
   as named in the corresponding `kubos-linux-build/board/kubos/{target}`
   directory.
-  -s : Sets the size of the rootfs.img file, specified in KB. The
   default is 13000 (13MB).
-  -i : Sets the name and location of the input `\*.its` file. Use if you
   want to create a custom package. The default is *kpack.its*.
-  -o {folder} : Specifies the name of the buildroot output folder. The
   default is 'output'
-  -v : Sets the version information for the package. The output file
   will be `kpack-{version}.itb`.
-  -b {branch} : Specifies the branch name of U-Boot that has been
   built. The default is 'master'. This option should not need to be
   used outside of development. U-Boot contains files which are used in
   the package generation process.

For example:

::

    $ ./kubos-package.sh -s 15000 -i /home/test/custom.its -v 2.0

Distribute the Package
~~~~~~~~~~~~~~~~~~~~~~

There isn't currently a central storage location or procedure for
non-release upgrade packages. 
This section should be upgraded once something has been implemented.
