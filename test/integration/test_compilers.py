import unittest
import re
import subprocess

class TestCompilers(unittest.TestCase):
    def test_latest_kubos_installed(self):
        bashCommand = "vagrant ssh -c 'mspdebug --version'"
        process = subprocess.Popen(bashCommand.split())
        output, error = process.communicate()
        regex = re.compile(r"MSPDebug version 0.24")
        self.assertTrue(regex.search( output ))

if __name__ == '__main__':
    unittest.main()
