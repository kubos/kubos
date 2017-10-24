Upgrading KubOS Linux
=====================

Overview
--------

KubOS Linux upgrades are distributed as kpack-{YYYY.MM.DD}.itb files.
ITB stands for 'Image Tree Blob' and allows Kubos to utilize the
existing DFU utility currently available in U-Boot.

Within each file will be a new version of the kernel image and root
filesystem.

To upgrade a board currently running KubOS Linux, an upgrade package
will be loaded into the upgrade partition of the board. For now, this
can be done through the Kubos SDK or by manually copying the package
into the upgrade partition.

Once the board is rebooted, U-Boot will take the package and then
install each component into the appropriate partition (kernel/rootfs).
If installation completes successfully, then the board will be rebooted
and then boot into the new version of KubOS Linux.

The overall flow looks like this:

.. figure:: images/kubos-linux-upgrade.png
   :alt: KubOS Linux Upgrade

   KubOS Linux Upgrade

**Note**:

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
be on and running KubOS Linux.

A Kubos SDK VM should be installed on your host computer and at least
one shared folder should be set up. Installation instructions can be
found :doc:`here <sdk-installing>`.

Installation
~~~~~~~~~~~~

Acquire an upgrade package. For now, this will likely be sent via email
from a Kubos engineer. Once an official distribution process has been
created this document will be upgraded with the new procedure.

Load the package into a shared folder accessible by your Kubos SDK VM.

Create or navigate to a Kubos SDK project. The content of the project
does not matter; it will only be used to flash the package correctly
onto the target.

::

    $ kubos init -l fakeproj
    $ cd fakeproj

Set the target to the desired KubOS Linux target type. 

For example::

    $ kubos target kubos-linux-isis-gcc

Build the project. This does not need to complete successfully. The
build process just brings in some files and settings that are required
in order to flash files to the board.

::

    $ kubos build

Use the kubos flash command to load the package onto your board. Note:
You might need to update your config.json file with the appropriate
login information to access your board. See the section :ref:`updating-credentials`
for more information.

::

    $ kubos flash /home/vagrant/shared/kpack-{version}.itb

Wait for the transfer to complete. This can take roughly 30 minutes. The
transfer rate via serial connection is slow because a) the upgrade
packages are large and b) it runs some CRC functions during transfer to
ensure that the package does not become corrupted. You should see a
progress bar indicating the time remaining for the transfer.

::

    Bytes Sent: 693248/1769379 BPS:8343 ETA 02:08

Refer to the :ref:`flash-troubleshooting` section if anything goes wrong
with the transfer.

Once the transfer has completed successfully, trigger a reboot of the
board. This can be done with the Linux ``reboot`` command. Once job
scheduling has been implemented, you will be able to schedule the
desired reboot time.

When the board boots into U-Boot, the new package will be detected and
loaded. If the loading is successful, the board will reboot into the
newly installed KubOS Linux. The U-Boot console messages will look
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

If for some reason you need to rollback to a previous version of KubOS
Linux, you don't need to reflash the board with the correct upgrade
package. Previous packages are not deleted once they have been loaded.
As a result, you can simply specify which package you would like to boot
into and then restart your system.

From the KubOS Linux shell:

::

    $ fw_setenv kubos_updatefile kpack-{desired version}.itb
    $ reboot

.. _upgrade-creation:

Upgrade Creation
----------------

This section is for developers who have made changes to KubOS Linux and
want to generate an upgrade package.

Pre-requisite
~~~~~~~~~~~~~

Build the new OS. Refer to the :ref:`build-os` instructions.

Run the Packaging Script
~~~~~~~~~~~~~~~~~~~~~~~~

From the 'kubos-linux-build/tools' folder, run the kubos-package.sh
script. This will create the rootfs.img and kubos-kernel.itb files and
then use the kpack.its file to bundle them into an \*.itb file. This is
the file that will be distributed to customers when an upgrade is
needed.

The automatically generated naming convention for the package is
kpack-\ *yyyy*-*mm*-*dd*.itb

Custom Packages
^^^^^^^^^^^^^^^

If you'd like to customize the package, there are a few different
options available through the script:

-  -t {target} : **Required** Specifies the name of the target board,
   as named in the corresponding `kubos-linux-build/board/kubos/{target}`
   directory.
-  -s : Sets the size of the rootfs.img file, specified in KB. The
   default is 13000 (13MB).
-  -i : Sets the name and location of the input \*.its file. Use if you
   want to create a custom package. The default is *kpack.its*.
-  -o {folder} : Specifies the name of the buildroot output folder. The
   default is 'output'
-  -v : Sets the version information for the package. The output file
   will be kpack-{version}.itb.
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
upgrade packages, since no official packages have been created yet. This
section should be upgraded once something has been implemented.
