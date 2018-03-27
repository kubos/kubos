# TARGET_DIR=$(shell luajit -e "print(require('ffi').os..'-'..require('ffi').arch)")
TARGET_DIR=target

INCLUDES=\
	-I../../../apis/isis-trxvu-api/isis-trxvu-api/\
  -I../../../apis/isis-trxvu-api/\
	-I../../../hal/kubos-hal-linux/kubos-hal-linux/\
	-I../../../hal/kubos-hal-linux/\
	-I../../../hal/kubos-hal/kubos-hal/\
	-I../../../hal/kubos-hal/\
	-I../../../libcsp/include/\
	-I../../../\

${TARGET_DIR}/libtwxvu.so: twxvu.o
	mkdir -p ${TARGET_DIR}
	$(CC) -shared -o $@ $<

twxvu.o: src/twxvu.c
	$(CC) -Os -std=gnu99 ${INCLUDES} -o $@ -c -fpic $<

clean:
	rm -f twxvu.o
