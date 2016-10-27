import unittest
import re
import subprocess

class TestCLI(unittest.TestCase):
    def test_latest_kubos_installed(self):
        bashCommand = "vagrant ssh -c 'kubos update'"
        process = subprocess.Popen(bashCommand.split())
        output, error = process.communicate()
        regex = re.compile(r"All up to date!")
        self.assertTrue(regex.search( output ))

if __name__ == '__main__':
    unittest.main()
