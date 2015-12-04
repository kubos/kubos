using_module = $(if $(filter $(1),$(USEMODULE)),1,0)

ifeq ($(BOARD),native)
  USEMODULE += native_uart
  USEMODULE += $(BOARDTYPE)
endif

ifeq ($(call using_module,fatfs),1)
  USEMODULE += fs
  USEMODULE += fatfs_$(BOARD)
endif

ifeq ($(call using_module,fs),1)
  USEMODULE += newlib_syscalls_fs
  USEMODULE += uart_stdio
endif

