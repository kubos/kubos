#!/usr/bin/env python
import argparse
import subprocess
import os

GENERATE_TAGS = """
(cat {0}; 
echo "PROJECT_NUMBER={1}";
echo "OUTPUT_DIRECTORY={2}";
echo "GENERATE_HTML=NO";
echo "GENERATE_TAGFILE={3}/output.tag"
echo "EXCLUDE=source test";) | doxygen -"""

GENERATE_HTML = """
(cat {0}; 
echo "PROJECT_NUMBER={1}";
echo "OUTPUT_DIRECTORY={2}";
echo "GENERATE_HTML=YES";
echo "TAGFILES={3}";
echo "HTML_OUTPUT=.";
echo "EXCLUDE=source test";) | doxygen -"""

DOC_TAG_DIR = """
{0}/output.tag={1}
"""

DOCS_DIRS = [
".",
"kubos-core", 
"libcsp", 
"freertos/os", 
"hal/isis-iobc-hal", 
"hal/kubos-hal-msp430f5529", 
"hal/kubos-hal", 
"hal/kubos-hal-stm32f4",
"services/telemetry/telemetry",
"services/telemetry/telemetry-linux",
"telemetry-aggregator",
"telemetry-storage",
"ipc"]

def gendocs_plain(dir, doxyfile, version, doc_dir):
    doxycmd = GENERATE_TAGS.format(doxyfile, version, doc_dir, doc_dir)
    subprocess.call((doxycmd), shell=True, cwd=dir)

def gendocs_tags(dir, doxyfile, version, doc_dir, tags_str):
    doxycmd = GENERATE_HTML.format(doxyfile, version, doc_dir, tags_str)
    subprocess.call((doxycmd), shell=True, cwd=dir)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', metavar='output', default='html',
                        help='Specifies output directory for docs')
    parser.add_argument('--version', metavar='version', default='0.0.0',
                        help='Specifies release version')

    args = parser.parse_args()

    doc_tags = []

    for dir in DOCS_DIRS:
        doc_dir = os.path.join(os.getcwd(), args.output, dir)
        if not os.path.isdir(doc_dir):
            os.makedirs(doc_dir)
        gendocs_plain(dir, "docs/Doxyfile", args.version, doc_dir)
        doc_tags.append(DOC_TAG_DIR.format(doc_dir, doc_dir).strip())

    doc_tags_str = " \\\n".join(doc_tags)
    for dir in DOCS_DIRS:
        doc_dir = os.path.join(os.getcwd(), args.output, dir)
        if not os.path.isdir(doc_dir):
            os.makedirs(doc_dir)
        gendocs_tags(dir, "docs/Doxyfile", args.version, doc_dir, doc_tags_str)


if __name__ == '__main__':
    main()