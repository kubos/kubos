#!/usr/bin/env python
import argparse
import json
import os
import subprocess
import sys
import sh

this_dir = os.path.abspath(os.path.dirname(__file__))
root_dir = os.path.dirname(this_dir)

repo_dirs = []

create_tag = []
update_tag = []
leave_tag = []


for root, dirs, files in os.walk(root_dir):
    for d in dirs:
        if d == '.git':
            if ".repo" not in root:
                repo_dirs.append(root)


def cmd(*args, **kwargs):
    cwd = kwargs.get('cwd', os.getcwd())
    print ' '.join(args)
    output = ''
    try:
        output = subprocess.check_output(args, **kwargs)
    except subprocess.CalledProcessError, e:
        pass
    return output


def link_app(app_dir, link_cmd):
    print '[app %s]' % app_dir
    for module_dir in module_dirs:
        cmd('yotta', link_cmd, module_name(module_dir), cwd=app_dir)

    for target_dir in target_dirs:
        cmd('yotta', link_cmd + '-target', target_name(target_dir), cwd=app_dir)


def get_last_tag(_dir):
    try:
        sh.cd(_dir)
        tag_list = sh.git("show-ref", "--tags")
        last_tag = sh.tail(tag_list, "-1")
        tag_sha = sh.awk(last_tag, "{print $1}")
    except:
        tag_sha = ""
    return tag_sha.strip()

def check_changes(_dir):
    sha = get_last_tag(_dir)
    if sha is "":
        create_tag.append(_dir)
    else:
        sh.cd(_dir)
        commit_log = sh.git("--no-pager", "log", "%s..HEAD" % sha, "--oneline")
        commit_log.strip()
        if len(commit_log) == 0:
            leave_tag.append(_dir)
        else:
            update_tag.append(_dir)

def main():

    for repo_dir in repo_dirs:
        check_changes(repo_dir)

    print "Create tags here"
    for _dir in create_tag:
        print "\t%s" % _dir

    print "Update tags here"
    for _dir in update_tag:
        print "\t%s" % _dir

    print "Leave tags alone"
    for _dir in leave_tag:
        print "\t%s" % _dir


if __name__ == '__main__':
    main()
