#!/usr/bin/env python
import argparse
import json
import os
import subprocess
import sys

from kubos_build import KubosBuild

this_dir = os.path.abspath(os.path.dirname(__file__))
root_dir = os.path.dirname(this_dir)

create_tag = []
update_tag = []
leave_tag = []

def check_changes(project):
    sha = project.get_last_tag()
    if sha is "":
        create_tag.append(project.path)
    else:
        commit_log = subprocess.check_output(["git", "--no-pager", "log",
                                              "%s..HEAD" % sha, "--oneline"],
                                             cwd=project.path)
        commit_log.strip()
        if len(commit_log) == 0:
            leave_tag.append(project.path)
        else:
            update_tag.append(project.path)

def main():
    kb = KubosBuild()
    for project in kb.projects:
        check_changes(project)

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
