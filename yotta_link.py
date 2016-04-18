#!/usr/bin/env python
import argparse
import json
import os
import subprocess
import sys

this_dir = os.path.abspath(os.path.dirname(__file__))
root_dir = os.path.dirname(this_dir)

module_dirs = []
target_dirs = []
example_dirs = []
for root, dirs, files in os.walk(root_dir):
    if 'yotta_targets' in root or 'yotta_modules' in root:
        continue

    for f in files:
        if f == 'module.json':
            if 'examples' in root:
                example_dirs.append(root)
            else:
                module_dirs.append(root)
        elif f == 'target.json':
            target_dirs.append(root)

def cmd(*args, **kwargs):
    cwd = kwargs.get('cwd', os.getcwd())
    print ' '.join(args)
    try:
        subprocess.check_call(args, **kwargs)
    except subprocess.CalledProcessError, e:
        print >>sys.stderr, 'Error executing command, giving up'
        sys.exit(1)

def module_name(module_dir):
    return json.load(open(os.path.join(module_dir, 'module.json')))['name']

def target_name(target_dir):
    return json.load(open(os.path.join(target_dir, 'target.json')))['name']

def link_sys(link_cmd):
    for module_dir in module_dirs:
        print '[module %s@%s]' % (module_name(module_dir), module_dir)
        cmd('yotta', link_cmd, cwd=module_dir)

    for target_dir in target_dirs:
        print '[target %s@%s]' % (target_name(target_dir), target_dir)
        cmd('yotta', link_cmd + '-target', cwd=target_dir)

def link_app(app_dir, link_cmd):
    print '[app %s]' % app_dir
    for module_dir in module_dirs:
        cmd('yotta', link_cmd, module_name(module_dir), cwd=app_dir)

    for target_dir in target_dirs:
        cmd('yotta', link_cmd + '-target', target_name(target_dir), cwd=app_dir)

def main():
    parser = argparse.ArgumentParser(
        description='Install or uninstall yotta symlinks for KubOS modules and ' \
                    'targets')

    parser.add_argument('--link', action='store_const', const='link',
                        default='link', help='install symlinks (default)')
    parser.add_argument('--unlink', dest='link', action='store_const',
                        const='unlink', help='uninstall symlinks')
    parser.add_argument('--sys', action='store_true',
                        help='install/uninstall target and module symlinks ' \
                             'to /usr/local/lib/yotta_*')
    parser.add_argument('--app', metavar='APP_DIR',
                        help='install/uninstall target and module symlinks ' \
                             'to APP_DIR')
    parser.add_argument('--all', action='store_true', default=False,
                        help='install/uninstall system symlinks and app ' \
                             'symlinks for local example apps (default)')

    args = parser.parse_args()
    if not args.sys and not args.app:
        args.all = True

    if args.sys:
        link_sys(args.link)

    if args.app:
        link_app(args.app, args.link)

    if args.all:
        if args.link == 'link':
            link_sys(args.link)
            for example_dir in example_dirs:
                link_app(example_dir, args.link)
        else:
            # unlink in reverse
            for example_dir in example_dirs:
                link_app(example_dir, args.link)
            link_sys(args.link)

if __name__ == '__main__':
    main()
