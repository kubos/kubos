#!/usr/bin/env python3
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
    return subprocess.call((doxycmd), shell=True, cwd=dir)

def sphinx_build(outformat, srcdir='docs/', outdir=None, verbose=0):
    if not outdir:
        outdir = outformat + '/'

    args = ['sphinx-build']
    if verbose > 0:
        args.append('-' + ('v' * verbose))

    if outformat == 'pdf':
        args.extend(['-b', 'latex'])

    args.extend(['-D', 'outformat=%s' % outformat, srcdir, outdir])

    if verbose > 0:
        print(" ".join(args))

    return subprocess.call(args)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--format', metavar='format', default='all',
                        help='Which docs to generate: html, pdf, cargo, or all')
    parser.add_argument('--output', metavar='output', default='xml',
                        help='Specifies output directory for docs')
    parser.add_argument('--version', metavar='version', default='0.0.0',
                        help='Specifies release version')

    args = parser.parse_args()

    if args.format not in ('html', 'pdf', 'cargo', 'all'):
        parser.error('Unknown doc format: {}'.format(args.format))

    doc_tags = {}

    doc_dirs = [d for d in DOCS_DIRS if os.path.isdir(d)]

    try:
        for dir in doc_dirs:
            doc_dir = os.path.join(os.getcwd(), args.output, dir)
            if not os.path.isdir(doc_dir):
                os.makedirs(doc_dir)

            result = gendocs_xml(dir, "docs/Doxyfile", args.version, doc_dir)
            if result != 0:
                raise Exception(result, 'Error generating Doxygen in %s' % dir)

        if args.format in ('html', 'all'):
            result = sphinx_build('html')
            if result != 0:
                raise Exception(result, 'Error generating HTML')

        if args.format in ('pdf', 'all'):
            result = sphinx_build('pdf')
            if result != 0:
                raise Exception(result, 'Error generating LaTeX')

            result = subprocess.call('make', cwd='pdf')
            if result != 0:
                raise Exception(result, 'Error transforming LaTeX to PDF')


        if args.format in ('cargo', 'all'):
            result = subprocess.call("cargo doc --all --no-deps", shell=True)
            if result != 0:
                raise Exception(result, 'Error generating cargo docs')

            dir_util.copy_tree("target/doc", "html/rust-docs")
    except Exception as e:
        parser.exit(*e.args)
    finally:
        if os.path.isdir("./xml"):
            shutil.rmtree("./xml")



if __name__ == '__main__':
    main()
