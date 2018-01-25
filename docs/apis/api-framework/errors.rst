Errors
======

It is normal for the API to encounter errors when interacting with hardware. We have made an effort to create a few error categories to allow the system to react predictably to these errors. 


Lack of Capability
------------------

Integrated Hardware, even ones that serve similar purposes, can have drastically different capabilities. Due to this, the API frameworks will often have functionality that not all hardware can fulfill. To make sure the :doc:`hardware services <../../services/hardware-services>` can still operate on those less capable components, we maintain that the API call should be present, but have it return a known error code to indicate that the unit cannot perform it. This error code is:

