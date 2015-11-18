APPLICATION = kubos-core

# If no BOARD is found in the environment, use this default:
BOARD ?= native
BOARDTYPE ?= beaglebone
DEBUG ?= 0

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../RIOT
RIOTBASE := $(abspath $(RIOTBASE))

# Absolute path to kubos modules base
KUBOS_MODULES ?= $(CURDIR)/modules
KUBOS_MODULES := $(abspath $(KUBOS_MODULES))

# Uncomment this to enable code in RIOT that does safety checking
# which is not needed in a production environment but helps in the
# development process:
ifneq ($(DEBUG),0)
CFLAGS += -g -DSCHEDSTATISTICS -DDEVELHELP
endif

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

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

KUBOS_USEMODULES := gps ham klog

ifneq ($(BOARD),native)
USEMODULE += newlib
endif

include $(KUBOS_MODULES)/Makefile.include
include $(RIOTBASE)/Makefile.include
