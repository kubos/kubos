node("raspi") {
  sh 'echo "THIS IS THE BUILD"'
  git url: 'https://github.com/kubostech/kubos.git'
  print env.JOB_NAME.replaceFirst('.+/', '')
  sh 'git checkout ' + env.JOB_NAME.replaceFirst('.+/', '')
  def workspace = pwd()
  sh "PYTHONPATH=\$(pwd) python ${workspace}/test/integration/jenkinsnode/tests/msp430f5529-test.py"
}
