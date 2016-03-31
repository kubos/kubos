MODULE = fatfs

ifeq (,$(wildcard $(CURDIR)/$(BOARD)))
$(error No FatFs implementation for board $(BOARD) found)
endif

DIRS += $(BOARD)

CFLAGS += -I$(KUBOS_MODULES)/fs -I$(CURDIR)/$(BOARD)

include $(RIOTBASE)/Makefile.base
