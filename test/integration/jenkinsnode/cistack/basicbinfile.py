import os
import sys
import magic
import re
import subprocess
from time import sleep
from utils import *
from binfile import Binfile

class Basicbinfile(Binfile):
    def __init__(self, name = "", path = "", board = ""):
        self.name = name
        self.path = path
        self.board = board

#<EOF>    
