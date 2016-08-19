#!/usr/bin/python
from collections import namedtuple
import os
import urlparse
import xml.dom.minidom as minidom
import string
from kubos_build import KubosBuild


def generate_xml(xml_projects, docker=False):
    doc = minidom.getDOMImplementation().createDocument(None, 'manifest', None)
    root = doc.documentElement

    sort_key = lambda a: a.name
    for remote in sorted(xml_remotes, key=sort_key):
        el = doc.createElement('remote')
        el.setAttribute('name', remote.name)
        el.setAttribute('fetch', remote.fetch)
        root.appendChild(el)

    el = doc.createElement('default')
    el.setAttribute('remote', DEFAULT_REMOTE)
    el.setAttribute('revision', DEFAULT_REVISION)
    root.appendChild(el)

    for project in sorted(xml_projects, key=sort_key):
        el = doc.createElement('project')
        el.setAttribute('name', project.name)
        if docker:
            if 'targets' in project.path:
                el.setAttribute('path', string.replace(project.path, 'targets', 'yotta_targets'))
            elif 'examples' in project.path:
                el.setAttribute('path', project.path)
            else:
                el.setAttribute('path', 'yotta_modules/' + project.name)
        else:
            el.setAttribute('path', project.path)
        el.setAttribute('revision', project.tag)
        root.appendChild(el)
    return doc

DEFAULT_REMOTE = 'kubostech'
DEFAULT_REVISION = 'master'
DEFAULT_FETCH = 'https://github.com/kubostech/'

Remote = namedtuple('Remote', ['name', 'fetch'])
Project = namedtuple('Project', ['name', 'path', 'revision'])

xml_remotes = set([Remote(name=DEFAULT_REMOTE, fetch=DEFAULT_FETCH)])
xml_projects = set()

kb = KubosBuild()
for project in kb.projects:
    project_args = dict(name=project.name, path=project.relpath)
    project_args['revision'] = project.tag
    xml_projects.add(Project(**project_args))


with open('default.xml', 'w') as manifest:
    doc = generate_xml(xml_projects, docker=False)
    manifest.write(doc.toprettyxml())
    print doc.toprettyxml()


with open('docker-manifest.xml', 'w') as manifest:
    doc = generate_xml(xml_projects, docker=True)
    manifest.write(doc.toprettyxml())
    print doc.toprettyxml()
