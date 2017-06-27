FatFS API
=========

This API provides an implementation of the `FatFS filesystem <http://elm-chan.org/fsw/ff/00index_e.html>`__.
It creates an easy way to interact with files on a connected media device.

In order to use this API, the :json:object:`fatfs <fs.fatfs>` object must be present in the project's `config.json` file.
Additionally, either the :json:object:`SPI <fs.fatfs.driver.spi>` or :json:object:`SDIO <fs.fatfs.driver.sdio>` child
object should be included to take full advantage of the library. An example is given below:

::

    {
        "fs": {
            "fatfs": {
                "driver": {
                    "sdio": {}
                }
            }
        }
    }
    
This would enable the FatFS API to interface with a direct SDIO connection to an SD card.

See the :ref:`sd-example` for an example of how to use this interface.

.. doxygengroup:: KUBOS_CORE_FF
    :project: kubos-core
    :members:
    :content-only: 