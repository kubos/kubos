Working with KubOS and an OBC
=============================

After getting your :doc:`development environment <../getting-started/local-setup>` set up and playing
around with the :doc:`basics of KubOS <../tutorials/index>`, you can move on to interacting with
hardware.

In this section, you'll find the following docs:

- :doc:`cross-compile` - Details the process required to compile code (C, Rust, etc) for a target
  OBC rather than your host computer
- :doc:`comms-setup` - Covers the possible ways to communicate with an OBC (serial, ethernet, etc)
  and how to set up the necessary connections
- :doc:`bbb/index` - Documentation specifically geared for the Beaglebone Black. Includes
  installation instructions and a system overview doc
- :doc:`iobc/index` - Documentation specifically geared for the Pumpkin MBM2. Includes installation
  instructions and a system overview doc
- :doc:`mbm2/index` - Documentation specifically geared for the ISIS OBC. Includes installation
  instructions and a system overview doc
- :doc:`porting-kubos` - A high-level guide of how to set up KubOS for a currently unsupported OBC

.. toctree::
    :hidden:
    
    cross-compile
    comms-setup
    bbb/index
    iobc/index
    mbm2/index
    porting-kubos