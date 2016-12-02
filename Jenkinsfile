node("raspi") {
  sh 'echo "THIS IS THE BUILD"'
  git url: 'https://github.com/kubostech/kubos.git'
  def workspace = pwd()
  sh "python ${workspace}/test/integration/test_raspi.py"
}
