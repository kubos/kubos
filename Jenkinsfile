node("raspi") {
  sh 'echo "THIS IS THE BUILD"'
  git url: 'https://github.com/kubostech/kubos.git'
  sh 'python test/integration/test_raspi.py'
}
