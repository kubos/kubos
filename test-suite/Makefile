APPLICATION = kubos-test

KUBOS_CORE ?= $(abspath $(CURDIR)/..)

# If no BOARD is found in the environment, use this default:
BOARD     ?= native
BOARDTYPE ?= beaglebone

# Modules to include:
USEMODULE += auto_init
USEMODULE += xtimer
USEMODULE += embunit
USEMODULE += gnrc_pktbuf
USEMODULE += gnrc_netif
USEMODULE += gnrc_netapi
USEMODULE += gnrc_netreg

# Kubos modules
USEMODULE += ham
USEMODULE += gps
USEMODULE += klog

ifneq ($(BOARD),native)
USEMODULE += fatfs
endif

CFLAGS += -g
INCLUDES += -I$(RIOTBASE)/sys/include

include $(KUBOS_CORE)/Makefile.include
