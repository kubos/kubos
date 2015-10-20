####
#### Sample Makefile for building applications with the RIOT OS
####
#### The example file system layout is:
#### ./application Makefile
#### ../../RIOT
####

# Set the name of your application:
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

# If you want to use native with valgrind, you should recompile native
# with the target all-valgrind instead of all:
# make -B clean all-valgrind

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
#USEMODULE += posix
#USEMODULE += newlib
#USEMODULE += location
USEMODULE += gnrc_pktbuf
USEMODULE += gnrc_netif
USEMODULE += gnrc_netapi
USEMODULE += gnrc_netreg

KUBOS_USEMODULES := gps radio ham

ifeq ($(BOARD),native)
USEMODULE += native_uart
KUBOS_USEMODULES += $(BOARDTYPE)
endif

# Add Kubos Modules to necessary Make variables
USEMODULE += $(KUBOS_USEMODULES)
KUBOS_MODULE_DIRS = $(foreach module,$(KUBOS_USEMODULES),$(CURDIR)/modules/$(module))
EXTERNAL_MODULE_DIRS += $(KUBOS_MODULE_DIRS)
CFLAGS += $(foreach dir,$(KUBOS_MODULE_DIRS),-I$(dir))
LINKFLAGS += -lm

include $(KUBOS_MODULES)/Makefile.include
include $(RIOTBASE)/Makefile.include

# ... and define them here (after including Makefile.include,
# otherwise you modify the standard target):
#proj_data.h: script.py data.tar.gz
#	./script.py

