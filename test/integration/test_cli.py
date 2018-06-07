import unittest
import re
import subprocess

class TestCLI(unittest.TestCase):
    def test_latest_kubos_installed(self):
        process = subprocess.Popen(["vagrant", "ssh", "-c", "kubos version"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE)
        output, error = process.communicate("n\n")
        regex = re.compile(r"Kubos-CLI version")
        print(output)
        self.assertTrue(regex.search(output))


if __name__ == '__main__':
    unittest.main()
