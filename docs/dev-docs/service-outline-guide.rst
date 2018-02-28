KubOS Service Outlines
=======================

This guide covers the development of KubOS hardware services and provides an outline for several major types of hardware. 
For general information about hardware services, their role, and how they work, check out :doc:`the hardware services documentation. <../services/hardware-services>`


General Hardware Service
-------------------------

A general hardware service is a service for any piece of hardware that does not fit into any of the other categories. These queries/mutations will be expected to be present regardless of the hardware. All other service outlines build on top of what is present here. 

Mutations::

    no