from collections import namedtuple
import os
import urlparse
import xml.dom.minidom as minidom

from kubos_build import KubosBuild

DEFAULT_REMOTE = 'kubostech'
DEFAULT_REVISION = 'master'
DEFAULT_FETCH = 'https://github.com/kubostech/'

Remote = namedtuple('Remote', ['name', 'fetch'])
Project = namedtuple('Project', ['name', 'path', 'remote', 'revision'])

xml_remotes = set([Remote(name=DEFAULT_REMOTE, fetch=DEFAULT_FETCH)])
xml_projects = set()

kb = KubosBuild()
for project in kb.projects:
    project_args = dict(name=project.name, path=project.relpath)
    if project.upstream:
        upstream_url = list(urlparse.urlparse(project.upstream['url']))
        upstream_url[2] = os.path.dirname(upstream_url[2]) + '/'
        xml_remotes.add(Remote(name=project.upstream['remote'], fetch=urlparse.urlunparse(upstream_url)))
        project_args['remote'] = project.upstream['remote']
        project_args['revision'] = project.upstream['branch']
    else:
        project_args['remote'] = DEFAULT_REMOTE
        project_args['revision'] = project.commit

    xml_projects.add(Project(**project_args))

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
    el.setAttribute('path', project.path)
    el.setAttribute('remote', project.remote)
    el.setAttribute('revision', project.revision)
    root.appendChild(el)

print doc.toprettyxml()
