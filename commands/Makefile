# Makefile template for shared library
# Run `make target="iobc" to build for the iboc

ifeq ($(target),iobc)
	CC:=arm-linux-gcc
else
	CC:=gcc # C compiler
endif

INCLUDE := -Iinclude #include directories
CFLAGS = -fPIC -Wall -Wextra -g $(INCLUDE) # C flags
LDFLAGS = -shared  # linking flags
RM = rm -f  # rm command
TARGET_LIB = libcore.so # target lib

SRCS = $(wildcard source/*.c)
OBJS = $(SRCS:.c=.o)

.PHONY: all
all: ${TARGET_LIB} install

$(TARGET_LIB): $(OBJS)
	$(CC) ${LDFLAGS} -o $@ $^

source/%.o: %.c
	$(CC)  $(CFLAGS) -o $@ $^

install: ${TARGET_LIB}
	cp ${TARGET_LIB} ~

.PHONY: clean
clean:
	-${RM} ${TARGET_LIB} ${OBJS}
