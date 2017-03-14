# Makefile template for shared library

CC = gcc # C compiler
INCLUDE := -Iinclude #include directories
BUILD_DIR := build
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

$(BUILD_DIR)/%.o: %.c
	$(CC) $(CFLAGS) -o $@ $^
#include $(SRCS:.c=.o)

install: ${TARGET_LIB}
	cp ${TARGET_LIB} ~

.PHONY: clean
clean:
	-${RM} ${TARGET_LIB} ${OBJS}
