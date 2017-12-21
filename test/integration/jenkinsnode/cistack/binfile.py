import os
import sys
import re
import magic
from utils import supportedBoards

errstr = "*** ERROR (binfile)"

class Binfile(object):
    """
    A binary file object that contains the name, file type, path, 
    target architecture, and target board type. Internal methods 
    include file and path validation, attempts to determine target
    architecture and file type, and a series of variable return
    methods.
    """

    def __init__(self, name = '', path = '', board = ''):
        self.board = board
        self.name = name
        self.path = path
        self.arch = None
        self.filetype = None
        self.abspath = None


    def validate(self):
        """trap conditions with no / no matching arguments for 'board'"""

        print("Checking for supported board.")
        if self.board == "": 
            sys.exit("Unknown board type. Exiting.")

        supportedboards = supportedBoards()

        if not self.board in supportedboards:
            sys.exit("Board %s is not supported." % self.board)
            return False

        if not self.getpath():        
            sys.exit("%s unable to find binary file to upload in \
            specified path or current working directory %s. \
            Exiting now." % (errstr, str(array[0])))

        array = self.getfiletype()
        if not (array[0] or array[1]):
            return False

        self.arch = array[0]
        self.filetype = array[1]
        return True

# small methods:
    def path(self):
        return self.path

    def board(self):
        return self.board

    def name(self):
        return self.name

    def abspath(self):
        return os.path.join(self.path,self.name)

    def arch(self):
        return self.arch

    def filetype(self):
        return self.filetype

# Big effort to find path to binary:
    def getpath(self):
        """Try to determine the correct path and name of the binary"""
        cwd = os.getcwd()

# if there's neither a name nor a path, puke
        if (self.path == "" and self.name == ""):
            sys.exit("Missing name and missing path. Exiting.")

# if the path is specified but isn't a directory, puke
        if not os.path.isdir(self.path):
            sys.exit("%s unable to verify path %s" % (errstr, self.path))

# if there's no file at the specified path and binfile name, puke
        if not os.path.isfile(os.path.join(self.path, self.name)):
            sys.exit("%s unable to locate binary file in path %s" % 
            (errstr, self.path))

# if self.path is defined and seems to be a real path, start checking
        if os.path.isdir(self.path):
            if os.path.isfile(os.path.join(self.path, self.name)):
                return True

# If self.path is not present, check if binfile includes a path --
# if it does but no path is specified, it should have a path attached,
# or else binfile must be in the current working directory.
        if (os.path.isfile(self.name) and (self.path == "")):
            array = os.path.split(self.name)

# if it exists but the os.path.split gives nothing in the first array element,
# check to see if it's in the current working directory.
            if not os.path.exists(array[0]):
                print("Unable to determine path to binary from \
                input path %s." % array[0])
                self.path = cwd

# if there isn't anything preceding self.name, either it's in the cwd or
# else it's an error condition.
                if os.path.isfile(os.path.join(self.path, array[1])):
                    self.path = cwd
                    self.name = array[1]
                    return True
                else: 
                    return False

# if, on the other hand, splitting the name gives a usable path--good!
            if os.path.exists(array[0]):
                self.path = array[0]
                self.name = array[1]
                return True
                    
# if the path exists 
            else:
                sys.exit("%s unable to find binary file to upload in \
                specified path or current working directory %s. \
                Exiting now." % (errstr, str(array[0])))
                return False

        return False

# Try to determine architecture and board target from binary file:

# TODO find a better way to parse which board was the target; this isn't bad
# but it's hardly exact.
#
# NA-satbus: 'ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, not stripped'
# NA-satbus .bin: 'data'
# MSP430: 'ELF 32-bit LSB executable, TI msp430, version 1 (embedded), statically linked, not stripped'
# MSP430 .bin: 'HIT archive data'
# STM32F407-disco: 'ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, not stripped'
# STM32F407-disco .bin: 'data'
# Pyboard: 'ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, not stripped'
# Pyboard .bin: 'data'

    def getfiletype(self):
        """Use [magic] to get some information about the binary upload file."""
        d = magic.from_file(os.path.join(self.path,self.name))
        d = re.sub(', ',',',d)
        e = d.split(',')
        filetype = e[0]
        array = [False,False]
        if filetype == 'data':
            array = ['ARM','BIN']
        elif filetype == 'HIT archive data':
            array = ['MSP430', 'BIN']
        elif re.search('ELF',filetype):
            arch = e[1]
            if arch == 'ARM':
                array = ['ARM','ELF']
            elif arch == 'TI msp430':
                array = ['MSP430','ELF']
            else:
                pass
        else:
            pass

        return array

    def getInfo(self):
        """Write information about the binary to stdout."""   
        print("\n---------------------------------")
        print("Info found about the binary file submitted for upload:")
        print("Name: %s" % self.name) 
        print("Path: %s" % self.path)
        print("Complete path to file: %s" % self.abspath())
        print("Arch: %s" % self.arch) 
        print("File type: %s" % self.filetype)
        print("Board: %s\n" % self.board)
        print("---------------------------------")
        return True

#<EOF>
