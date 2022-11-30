#!/usr/bin/env python3
import argparse
import subprocess
import os
import shutil
from distutils import dir_util

this_dir = os.path.abspath(os.path.dirname(__file__))
kubos_dir = os.path.dirname(os.path.dirname(this_dir))

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

    doc_dirs = [d for d in DOCS_DIRS if os.path.isdir(os.path.join(kubos_dir, d))]

    for dir in doc_dirs:
        doc_dir = os.path.join(kubos_dir, args.output, dir)
        if not os.path.isdir(doc_dir):
            os.makedirs(doc_dir)
        gendocs_xml(os.path.join(kubos_dir, dir), "docs/Doxyfile", args.version, doc_dir)

    subprocess.call("poetry run sphinx-build ../docs/ ../html/", shell=True, cwd=os.path.dirname(this_dir))
    shutil.rmtree(os.path.join(kubos_dir, "xml"))

    subprocess.call("cargo doc --all --no-deps", shell=True, cwd=kubos_dir)
    dir_util.copy_tree(os.path.join(kubos_dir, "target/doc"), os.path.join(kubos_dir, "html/rust-docs"))

if __name__ == '__main__':
    main()
