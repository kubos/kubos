import codecs
import os
import sys
import urllib

from docutils import nodes
from docutils.parsers.rst import Directive
from docutils.parsers.rst import directives
from docutils.statemachine import StringList
from jinja2 import FileSystemLoader, Environment
import sphinx.util

# This extension modified from it's original at https://github.com/kenyon/sphinx-jinja

class JinjaDirective(Directive):
    has_content = True
    optional_arguments = 1
    option_spec = {
        "file": directives.path,
        "header_char": directives.unchanged,
        "debug": directives.unchanged,
    }
    app = None

    def run(self):
        node = nodes.Element()
        node.document = self.state.document
        env = self.state.document.settings.env
        docname = env.docname
        template_filename = self.options.get("file")
        debug_template = self.options.get("debug")
        cxt = (self.app.config.jinja_contexts[self.arguments[0]].copy()
               if self.arguments else {})
        cxt["options"] = {
            "header_char": self.options.get("header_char")
        }
        if template_filename:
            if debug_template is not None:
                print('')
                print('********** Begin Jinja Debug Output: Template Before Processing **********')
                print('********** From {} **********'.format(docname))
                reference_uri = directives.uri(template_filename)
                template_path = urllib.request.url2pathname(reference_uri)
                encoded_path = template_path.encode(sys.getfilesystemencoding())
                imagerealpath = os.path.abspath(encoded_path)
                with codecs.open(imagerealpath, encoding='utf-8') as f:
                    print(f.read())
                print('********** End Jinja Debug Output: Template Before Processing **********')
                print('')
            tpl = Environment(
                          loader=FileSystemLoader(
                              self.app.config.jinja_base, followlinks=True)
                      ).get_template(template_filename)
        else:
            if debug_template is not None:
                print('')
                print('********** Begin Jinja Debug Output: Template Before Processing **********')
                print('********** From {} **********'.format(docname))
                print('\n'.join(self.content))
                print('********** End Jinja Debug Output: Template Before Processing **********')
                print('')
            tpl = Environment(
                      loader=FileSystemLoader(
                          self.app.config.jinja_base, followlinks=True)
                  ).from_string('\n'.join(self.content))
        if debug_template is not None:
            print('')
            print('************* Context ****************')
            print('%s' % cxt)
            print('************* End Context ************')

        new_content = tpl.render(**cxt)
        if debug_template is not None:
            print('')
            print('********** Begin Jinja Debug Output: Template After Processing **********')
            print(new_content)
            print('********** End Jinja Debug Output: Template After Processing **********')
            print('')
        new_content = StringList(new_content.splitlines(), source='')
        sphinx.util.nested_parse_with_titles(
            self.state, new_content, node)
        return node.children


def setup(app):
    JinjaDirective.app = app
    app.add_directive('jinja', JinjaDirective)
    app.add_config_value('jinja_contexts', {}, 'env')
    app.add_config_value('jinja_base', os.path.abspath('.'), 'env')
    return {'parallel_read_safe': True, 'parallel_write_safe': True}
