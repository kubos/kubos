APPLICATION = kubos-test

# If no BOARD is found in the environment, use this default:
BOARD ?= native
BOARDTYPE ?= beaglebone

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../../RIOT
RIOTBASE := $(abspath $(RIOTBASE))

# Absolute path to kubos modules base
KUBOS_MODULES ?= $(CURDIR)/../modules
KUBOS_MODULES := $(abspath $(KUBOS_MODULES))

# Uncomment this to enable scheduler statistics for ps:
#CFLAGS += -DSCHEDSTATISTICS

# Uncomment this to enable code in RIOT that does safety checking
# which is not needed in a production environment but helps in the
# development process:
#CFLAGS += -DDEVELHELP
#CFLAGS += -DDEBUG

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

# Modules to include:
USEMODULE += auto_init
USEMODULE += xtimer
USEMODULE += embunit
USEMODULE += gnrc_pktbuf
USEMODULE += gnrc_netif
USEMODULE += gnrc_netapi
USEMODULE += gnrc_netreg

KUBOS_USEMODULES := radio ham gps klog

CFLAGS += -g
INCLUDES += -I$(RIOTBASE)/sys/include

include $(KUBOS_MODULES)/Makefile.include
include $(RIOTBASE)/Makefile.include
