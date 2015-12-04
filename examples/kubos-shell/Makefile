APPLICATION = kubos-shell

KUBOS_CORE ?= $(abspath $(CURDIR)/../..)

# If no BOARD is found in the environment, use this default:
BOARD     ?= native
BOARDTYPE ?= beaglebone

# Modules to include:
USEMODULE += shell
USEMODULE += shell_commands
USEMODULE += vtimer
USEMODULE += xtimer
USEMODULE += auto_init
USEMODULE += gnrc_pktbuf
USEMODULE += gnrc_netif
USEMODULE += gnrc_netapi
USEMODULE += gnrc_netreg
USEMODULE += gps
USEMODULE += ham
USEMODULE += klog

ifneq ($(BOARD),native)
USEMODULE += fatfs
endif

include $(KUBOS_CORE)/kubos.mk
