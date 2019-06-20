Setting up the Kubos Windows Development Environment
====================================================

There are a couple things which should be set up in order to ease the process of developing services
and applications for KubOS when using a Windows-based host machine.

Remote File Development
-----------------------

Since Windows does not support symlinks and the SDK does not expose a graphical interface, creating
and editting project files can be a pain, as they are only accessible to tools within the SDK such
as ``vim`` or ``nano``.
It is possible to set up a remote connection between Notepad++ (or an IDE of your choice) on the
host machine and your SDK instance, rather than being forced to devlop using these command line tools.

.. note::
 
	Before proceeding, please make sure you have :doc:`installed the SDK. <sdk-installing>`

How does it work?
~~~~~~~~~~~~~~~~~

These instructions will set up your host machine to treat the SDK like a remote machine, using an
automatic FTP plug-in to allow the user to view and edit files on the SDK as if they were being
edited locally.

These instructions use a chosen environment which consists of:

- Notepad++
- NppFTP Plugin

This same method can be used with many common IDEs that have FTP packages for working on remote servers.

Installation
~~~~~~~~~~~~

Install Notepad++ `here. <https://notepad-plus-plus.org/download/v7.4.2.html>`_
Unless you know what you're doing and want to use something else, choose the first option of the
installer: "Notepad++ Installer 32-bit x86". Choose all the default options in the installer
(unless, as it states, you know what you're doing).

Install the NppFTP plugin using the Plugin Manager.

- Go to "Plugins" -> "Plugin Manager" -> "Show Plugin Manager"
- Under "Available", find "NppFTP". Click the box next to it to select it, then select "Install".

.. note::

    It might prompt you to update the Plugin Manager before installing.
    We recommend doing this once.
    It will require a restart of Notepad++, and you will have to repeat all the steps.
    If it prompts again after the first time, select "No" and it should install normally.

- After Notepad++ has restarted, you should now see "NppFTP" as one of the options under "Plugins".

Setup
~~~~~

Find the Vagrant configuration parameters
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Go to the install location of the Kubos SDK and bring up your Vagrant.
As it initializes, it will output its configuration:

::

		$ vagrant up
		Bringing machine 'default' up with 'virtualbox' provider...
		==> default: Checking if box 'kubos/kubos-dev' is up to date...
		==> default: A newer version of the box 'kubos/kubos-dev' is available! You currently
		==> default: have version '0.2.3'. The latest is version '1.0.1'. Run
		==> default: `vagrant box update` to update.
		==> default: Clearing any previously set forwarded ports...
		==> default: Clearing any previously set network interfaces...
		==> default: Preparing network interfaces based on configuration...
		    default: Adapter 1: nat
		==> default: Forwarding ports...
		    default: 22 (guest) => 2222 (host) (adapter 1)
		==> default: Booting VM...
		==> default: Waiting for machine to boot. This may take a few minutes...
		    default: SSH address: 127.0.0.1:2222
		    default: SSH username: vagrant
		    default: SSH auth method: private key
		==> default: Machine booted and ready!
		==> default: Checking for guest additions in VM...
		==> default: Mounting shared folders...
		    default: /vagrant => C:/Users/jacof/Documents/git/kubos
		    default: /vagrant_data => C:/Users/jacof/Documents/git/kubos
		==> default: Machine already provisioned. Run `vagrant provision` or use the `--provision`
		==> default: flag to force provisioning. Provisioners marked to run always will still run.

Record the SSH address (127.0.0.1:2222) and the SSH username (vagrant).

If the VM is already up, you can also issue ``vagrant ssh-config`` to get the hostname and port info.

.. Note:: 
	If you update your Vagrant box, this information could change. 

Configure NppFTP to access the SDK
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

- Go to "Plugins" -> "NppFTP" -> "Show NppFTP Window".
  This should bring up the NppFTP windown on the right side.
- In the NppFTP window, go to "Settings" (the gear) -> "Profile Settings"
- Select "Add New" in the bottom left, and name it "Kubos SDK".
- Edit the settings to match the picture below. You'll need to input:

  + Hostname and Port from the SSH address recorded previously
  + Username: "vagrant"
  + Password: "vagrant"
  + Initial remote directory: "/home/vagrant/"
  + Connection type: SFTP

.. image:: ../images/NppFTP_config.PNG

Usage
~~~~~

Connect to the Vagrant box by selecting "(Dis)Connect" -> "Kubos SDK".
This should automatically pull up the file system of the Vagrant with the /home/vagrant directory open.
It should say "NppFTP - Connected to Kubos SDK" at the top of the NppFTP window.

Now you can open and edit files! Double clicking on a file in the file tree will open it locally.
If you make changes to any file, it will automatically tranfer the file over and replace it on the
host machine whenever you hit save.

.. _windows-udp:

Allowing UDP Communication
--------------------------

There are certain scenarios where the SDK needs to be able to receive UDP packets from an OBC when
connected via a local ethernet port.
For example, when using the :doc:`file transfer client <../tutorials/file-transfer>`.

In this case, Windows Firewall may need to be updated to allow this traffic.

1. Open 'Windows Firewall with Advanced Security'. You can find this program by opening the start
   menu and searching for "firewall"

.. image:: ../images/windows_firewall.png

2. Click on "Inbound Rules", then scroll down to the "VBoxHeadless" rules. Find the rule which blocks
   UDP traffic on Public networks.

.. image:: ../images/vbox_firewall_rule.png

3. Right-click on the rule and select "Disable Rule"

.. image:: ../images/vbox_firewall_rule_disable.png

4. Right-click on "Inbound Rules" and select "New Rule"

.. image:: ../images/inbound_new_rule.png

5. Select "Custom" for the type of rule

.. image:: ../images/inbound_rule_custom.png

6. Select "All programs"

.. image:: ../images/inbound_rule_programs.png

7. Select "UDP" as the protocol type. Leave the "Local port" and "Remote port" settings as "All Ports"

.. image:: ../images/inbound_rule_ports.png

8. Under "Which remote IP addresses does this rule apply to?", click "These IP addresses", then click
   "Add"

.. image:: ../images/inbound_rule_ip.png

9. In the "This IP address or subnet" field, add the IP address of your OBC, then click "OK", then
   click "Next"

.. image:: ../images/inbound_rule_new_ip.png

10. Select "Allow the connection"

.. image:: ../images/inbound_rule_connection.png

11. In the "When does this rule apply?" menu, leave all checkboxes selected

.. image:: ../images/inbound_rule_network.png

12. In the "Name" field, enter something descriptive for the rule. For example, "Allow UDP from OBC".
    Then click "Finish" to finalize and activate the new rule.

.. image:: ../images/inbound_rule_name.png