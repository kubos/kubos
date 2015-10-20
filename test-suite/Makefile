####
#### Sample Makefile for building applications with the RIOT OS
####
#### The example file system layout is:
#### ./application Makefile
#### ../../RIOT
####

# Set the name of your application:
APPLICATION = kubos-test

# If no BOARD is found in the environment, use this default:
BOARD ?= native

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../../RIOT
RIOTBASE := $(abspath $(RIOTBASE))

# Absolute path to kubos modules base
KUBOS_MODULES ?= $(CURDIR)/../modules
KUBOS_MODULES := $(abspath $(KUBOS_MODULES))

# Uncomment this to enable scheduler statistics for ps:
#CFLAGS += -DSCHEDSTATISTICS

# If you want to use native with valgrind, you should recompile native
# with the target all-valgrind instead of all:
# make -B clean all-valgrind

# Uncomment this to enable code in RIOT that does safety checking
# which is not needed in a production environment but helps in the
# development process:
#CFLAGS += -DDEVELHELP
#CFLAGS += -DDEBUG

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

# Modules to include:
#USEMODULE += shell
#USEMODULE += vtimer
USEMODULE += xtimer
#USEMODULE += posix
#USEMODULE += newlib
#USEMODULE += location
USEMODULE += radio
USEMODULE += ham
USEMODULE += embUnit
USEMODULE += gnrc_pktbuf
USEMODULE += gnrc_netif
USEMODULE += gnrc_netapi
USEMODULE += gnrc_netreg

CFLAGS += -g
INCLUDES += -I$(KUBOS_MODULES)/radio
INCLUDES += -I$(KUBOS_MODULES)/ham
INCLUDES += -I$(KUBOS_MODULES)/location
INCLUDES += -I$(RIOTBASE)/sys/include

include $(KUBOS_MODULES)/Makefile.include
include $(RIOTBASE)/Makefile.include

# ... and define them here (after including Makefile.include,
# otherwise you modify the standard target):
#proj_data.h: script.py data.tar.gz
#	./script.py

