#!/usr/bin/env python3

import subprocess
import os
import shutil

projects = [
    "./cmocka",
    "./ccan/json",
    "./examples/adc-thermistor",
    "./examples/kubos-linux-uarttx",
    "./examples/kubos-linux-uartrx",
    "./examples/kubos-linux-example",
    "./examples/kubos-linux-tcprx",
    "./examples/rust-c-service/extern-lib",
    "./examples/kubos-linux-tcptx",
    "./test/integration/linux/iobc-supervisor-test",
    "./test/integration/linux/isis-imtq",
    "./test/integration/linux/bme280-spi",
    "./test/integration/linux/isis-trxvu/radio-test",
    "./test/integration/linux/isis-ants",
    "./test/integration/linux/nanopower-p31u",
    "./test/integration/linux/lsm303dlhc-i2c",
    "./test/integration/linux/hello-world",
    "./hal/kubos-hal",
    "./apis/gomspace-p31u-api",
    "./apis/isis-ants-api",
    "./apis/isis-imtq-api",
    "./apis/isis-trxvu-api",
    "./apis/isis-iobc-supervisor",
]

def clean(dir):
    build_dir = "build"
    shutil.rmtree(build_dir, ignore_errors=True)

def build(dir):
    build_dir = "build"
    cmake_dir = "../{}".format(dir)
    os.mkdir(build_dir)
    subprocess.run(["cmake", cmake_dir], cwd=build_dir, check=True)
    subprocess.run(["make"], cwd=build_dir, check=True)

def run_test(dir):
    build_dir = "build"
    os.environ["CTEST_OUTPUT_ON_FAILURE"] = "1"
    subprocess.run(["make", "test"], cwd=build_dir, check=True)

def test(dir):
    test_dir = "{}/test".format(dir)
    if os.path.isdir(test_dir):
        clean(test_dir)
        build(test_dir)
        run_test(test_dir)

def main():
    print("Building C projects")
    for dir in projects:
        print("Testing {}".format(dir))
        clean(dir)
        build(dir)
        test(dir)

if __name__ == '__main__':
    main()

