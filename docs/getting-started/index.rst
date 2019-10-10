Getting Started with KubOS
==========================

.. toctree::
    :hidden:
    :titlesonly:
    
    no-board
    local-setup
    windows-setup
    using-rust
    using-python
    local-services

The **KubOS** platform provides satellite developers the tools and libraries necessary to quickly bring up space-ready software. The guides and docs referenced here will lead you through getting your environment setup and interacting with the core KubOS services.

Try KubOS
---------

The first step to trying out KubOS is getting the appropriate environment setup.

Local Environment
~~~~~~~~~~~~~~~~~

Ready to get started setting up the Kubos development environment? Running a Linux or Mac system?
This :doc:`guide <local-setup>` will walk you through the tools and libraries which need to be installed in order to build and run KubOS components.

Windows Environment
~~~~~~~~~~~~~~~~~~~

If you are running Windows on your local computer, then you will need to follow :doc:`this guide <windows-setup>` to get your
local development environment up and running.

SDK Environment
~~~~~~~~~~~~~~~

Prefer to use a VM rather than install our development dependencies locally?
You can also take advantage of our :doc:`VM-based SDK <../sdk-docs/index>` for an easy-to-setup, self-contained development environment.

.. _interacting-with-kubos:

Interacting with KubOS
----------------------

After getting the appropriate local environment setup, it is time to begin running and interacting with the KubOS system.

KubOS can support many different languages, however our preferred languages are Python and Rust.

Using Python
~~~~~~~~~~~~

Once your local or VM environment is setup, you are ready to begin developing and interacting with the KubOS system! Is Python your language of choice? Head over to the :doc:`Python guide <using-python>`.

Using Rust
~~~~~~~~~~

More interested in using Rust for development? Take a look at the :doc:`Rust guide <using-rust>`.

Using C
~~~~~~~

If you would like to develop in C, we highly recommend you :doc:`install the SDK <../sdk-docs/sdk-installing>`
and then refer to :doc:`../sdk-docs/sdk-c`.

Learn more about KubOS
----------------------

Once you are familiar with the basics of KubOS development and interaction, you may want to dig into deeper topics. These tutorials and guides will help you peel back the layers a bit.

Design
~~~~~~

Interested in seeing a high level explanation of KubOS? The :doc:`KubOS design doc <../kubos-design>` provides a great overview of the design behind KubOS.

Tutorials
~~~~~~~~~

Ready to dig deeper into KubOS development?

Want to get a closer look at creating mission applications? Take a look at our :ref:`mission-development-tutorials`.

Ready to build your mission in KubOS? Read over our :doc:`mission development guide <../mission-dev/index>` for our walkthrough.

Interested in how KubOS interacts with the ground? Take a look at our :ref:`system-interaction-tutorials`.

Want to dig even deeper into KubOS? Take a look at our :ref:`advanced-tutorials`.

Services
~~~~~~~~

Interested in learning more about the services which make KubOS go? Check out the :doc:`core services guide <../ecosystem/services/core-services>`.

Ready to begin developing your own service? Check out the :doc:`service development guide <../ecosystem/services/service-dev>`.

The KubOS core services provide the base functionality that all KubOS systems rely on and interact with.
The :doc:`local services guide <local-services>` will help you get the core services up and running in your development environment.

Something Missing?
------------------

If something is missing in the documentation or if you found some part confusing, please `file an issue in the kubos repo <https://github.com/kubos/kubos/issues/new/choose>`__ with your suggestion for improvement or `contact us on slack <https://slack.kubos.com>`__.