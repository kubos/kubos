# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(KUBOS_CORE)/../RIOT
RIOTBASE := $(abspath $(RIOTBASE))

RIOTBOARD ?= $(RIOTBASE)/boards
RIOTBOARD := $(abspath $(RIOTBOARD))

DEBUG ?= 0
QUIET ?= 1

ifneq ($(DEBUG),0)
  CFLAGS += -g -DSCHEDSTATISTICS -DDEVELHELP
endif

include $(KUBOS_CORE)/build/base.mk
include $(RIOTBASE)/Makefile.include

ifeq ($(call using_module,fs),1)
  include $(KUBOS_MODULES)/fs/syscalls/Makefile.include
endif
