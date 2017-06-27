System Logging API
==================

This API offers a bunch of ``LOG_*`` functions that, with the default
implementation, just use printf, but honor a verbosity level.

If desired, it is possible to implement a log module which then will be used
instead the default printf-based implementation.  In order to do so, the log
module has to

1. provide "log_module.h"
2. have a name starting with ``log_`` *or* depend on the pseudo-module LOG,
3. implement log_write()

.. doxygengroup:: KUBOS_CORE_LOG
    :project: kubos-core
    :members:
    :content-only: 