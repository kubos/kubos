#!/usr/bin/env python2
import argparse
import subprocess
import os
import shutil
from distutils import dir_util

GENERATE_XML = """
(cat {0};
echo "";
echo "PROJECT_NUMBER={1}";
echo "OUTPUT_DIRECTORY={2}";
echo "XML_OUTPUT=.";) | doxygen -"""

DOCS_DIRS = [
    "apis/gomspace-p31u-api",
    "apis/isis-ants-api",
    "apis/isis-imtq-api",
    "apis/isis-iobc-supervisor",
    "apis/isis-trxvu-api",
    "hal/kubos-hal",
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
    shutil.rmtree("./xml")
    
    subprocess.call("cargo doc -t x86-linux-native -- --no-deps", shell=True)
    dir_util.copy_tree("target/x86_64-unknown-linux-gnu/doc", "html/rust-docs")

if __name__ == '__main__':
    main()
