Kubos FAQs and Troubleshooting Tips
===================================

TODO: Figure out what to do with this. Do we want to keep it? Is it useful?

.. contents:: :local:

FAQs
----

How do I contact y'all?
~~~~~~~~~~~~~~~~~~~~~~~

Our community Slack: `slack.kubos.co <https://slack.kubos.co>`__

Our email: info@kubos.co

How do I set up the Kubos SDK?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

See :doc:`sdk-docs/sdk-installing`

How do I check if I'm using the latest version of the Kubos Vagrant image?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

From your host machine, issue the ``vagrant box outdated`` command.

How do I get the latest version of the Kubos Vagrant image?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

From your host machine, issue the ``vagrant box update`` command.

.. warning:: This will overwrite all files in your existing image


How do I see the debug output of my board?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``minicom kubos``

How do I check what version of Kubos Linux I'm running?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Issue the ``uname -r`` command on the target board to display the kernel information.
The Kubos Linux version will be the *n.n.n* number after "KubOS".

::

    uname -r
    4.4.23-KubOS-1.0.0

What's the default Kubos Linux login?
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

kubos/Kubos123

Troubleshooting
---------------

Kubos SDK
~~~~~~~~~

The ``vagrant up`` command just hangs
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If you're using Windows 7 SP1, make sure you are using the :ref:`correct version of
Windows PowerShell <powershell>`.

I've tried other steps here, but my Kubos Vagrant image is still behaving weirdly.
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Try logging out and restarting the VM using the ``vagrant reload`` command.

I can't build my project. I keep getting "Permission denied" errors
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If you copied your project from another location, it's possible that the files are set up with root permissions
only. Change the project file permissions to allow the local ``vagrant`` user to have access.

::

    sudo chown vagrant:vagrant . -R

Interacting with an OBC
~~~~~~~~~~~~~~~~~~~~~~~

Can't Connect via Serial Debug
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If the SDK was unable to connect to the Kubos Linux target using ``minicom kubos``:

-  Check that the Kubos Linux target is turned on and connected to your computer

-  Check that no other Vagrant images are running. Only one VM can have
   control of the USB, so it may be that another instance currently has
   control of the device. You can shutdown a Vagrant image with the
   command ``vagrant halt``

-  Verify that the USB is showing up within the Vagrant environment with
   the ``lsusb`` command. You should see an FTDI device

-  Verify that the USB has been mapped to a linux device. Issue the
   command ``ls /dev``. You should see a /dev/ttyUSB\* device. If you
   don't, try rebooting your Vagrant image (``vagrant halt``,
   ``vagrant up``)

Can't Connect via SSH
^^^^^^^^^^^^^^^^^^^^^

Log in to the board via the debug UART and verify the IP address matches what is expected with ``ipaddr``.

For more information, check out our documentation about :ref:`ethernet connections <ethernet>`.

System Won't Boot into Linux
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If the system goes through the :doc:`full recovery process <os-docs/linux-docs/kubos-linux-recovery>` and the bootcount is still exceeded,
it will present the U-Boot CLI instead of attempting to boot into Kubos Linux again.

If this occurs, follow the :ref:`instructions for resetting the boot environment <env-reset>`.

Note: This is a case which normal users should never encounter, but becomes more likely when initially testing
custom Kubos Linux builds.


I transferred a script, but it won't run
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

``scp`` does not preserve file modes by default, however ``scp -p`` should preserve
the execute bit. Check that your file has the appropriate execute permissions turned on.
