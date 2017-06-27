#!/usr/bin/env python
import argparse
import subprocess
import os

GENERATE_XML = """
(cat {0};
echo ""; 
echo "PROJECT_NUMBER={1}";
echo "OUTPUT_DIRECTORY={2}";
echo "XML_OUTPUT=.";) | doxygen -"""

# DOCS_DIRS = [
# "kubos-core", 
# "libcsp", 
# "freertos/os", 
# "services/telemetry/telemetry",
# "services/telemetry/telemetry-linux",
# "telemetry-aggregator",
# "telemetry-storage",
# "ipc"]

DOCS_DIRS = [
    "hal/kubos-hal",
    "hal/kubos-hal-iobc",
    "hal/kubos-hal-msp430f5529",
    "hal/kubos-hal-stm32f4"
    "services/telemetry/telemetry",
    "services/telemetry/telemetry-linux"
]

def gendocs_xml(dir, doxyfile, version, doc_dir):
    doxycmd = GENERATE_XML.format(doxyfile, version, doc_dir)
    subprocess.call((doxycmd), shell=True, cwd=dir)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', metavar='output', default='xml',
                        help='Specifies output directory for docs')
    parser.add_argument('--version', metavar='version', default='0.0.0',
                        help='Specifies release version')

    args = parser.parse_args()

    doc_tags = {}

    doc_dirs = [d for d in DOCS_DIRS if os.path.isdir(d)]


    for dir in doc_dirs:
        doc_dir = os.path.join(os.getcwd(), args.output, dir)
        if not os.path.isdir(doc_dir):
            os.makedirs(doc_dir)
        gendocs_xml(dir, "docs/Doxyfile", args.version, doc_dir)

    subprocess.call("sphinx-build docs/ html/", shell=True)


if __name__ == '__main__':
    main()