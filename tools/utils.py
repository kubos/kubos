import subprocess
import os
import sys

def cmd(*args, **kwargs):
    cwd = kwargs.get('cwd', os.getcwd())
    save_output = kwargs.pop('save_output', False)
    echo = kwargs.pop('echo', True)

    if echo:
        print ' '.join(args)
    try:
        if save_output:
            return subprocess.check_output(args, **kwargs)
        else:
            return subprocess.check_call(args, **kwargs)
    except subprocess.CalledProcessError, e:
        if echo:
            print >>sys.stderr, 'Error executing command, giving up'
        return 1