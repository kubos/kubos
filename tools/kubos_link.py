#!/usr/bin/env python2
import argparse
import json
import os
import subprocess
import sys
import utils

from kubos_build import KubosBuild

this_dir = os.path.abspath(os.path.dirname(__file__))
root_dir = os.path.dirname(this_dir)

class KubosLinker(object):
    def __init__(self):
        self.kb = KubosBuild()

    def link_sys(self, link_cmd):
        for module in self.kb.modules(include_bin=False):
            print '[module %s@%s]' % (module.yotta_name(), module.path)
            utils.cmd('kubos', link_cmd, cwd=module.path)

        for target in self.kb.targets():
            print '[target %s@%s]' % (target.yotta_name(), target.path)
            utils.cmd('kubos', link_cmd + '-target', cwd=target.path)

    def link_app(self, app_dir, link_cmd):
        print '[app %s]' % app_dir
        for module in self.kb.modules(include_bin=False):
            utils.cmd('kubos', link_cmd, module.yotta_name(), cwd=app_dir)

        for target in self.kb.targets():
            utils.cmd('kubos', link_cmd + '-target', target.yotta_name(),
                     cwd=app_dir)

def main():
    parser = argparse.ArgumentParser(
        description='Install or uninstall kubos symlinks for KubOS modules and ' \
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
    parser.add_argument('--modules', action='store_true', default=False,
                        help='install/uninstall target/modulesystem symlinks ' \
                             'for all modules within the kubos tree')
                        

    args = parser.parse_args()
    if not args.sys and not args.app:
        args.all = True

    linker = KubosLinker()
    if args.sys:
        linker.link_sys(args.link)

    if args.app:
        linker.link_app(args.app, args.link)

    if args.all:
        if args.link == 'link':
            linker.link_sys(args.link)
            for mod in linker.kb.bin_modules():
                linker.link_app(mod.path, args.link)
        else:
            # unlink in reverse
            for mod in linker.kb.bin_modules():
                linker.link_app(mod.path, args.link)
            linker.link_sys(args.link)

    if args.modules:
        if args.link == 'link':
            linker.link_sys(args.link)
            for mod in linker.kb.modules():
                linker.link_app(mod.path, args.link)
        else:
            # unlink in reverse
            for mod in linker.kb.modules():
                linker.link_app(mod.path, args.link)
            linker.link_sys(args.link)

if __name__ == '__main__':
    main()
