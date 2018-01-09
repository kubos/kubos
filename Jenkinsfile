node("raspi") {
  sh 'echo "THIS IS THE BUILD"'
  git url: 'https://github.com/kubos/kubos.git'
  print env.JOB_NAME.replaceFirst('.+/', '')
  sh 'git checkout ' + env.JOB_NAME.replaceFirst('.+/', '')
  def workspace = pwd()
  sh "python ${workspace}/test/integration/jenkinsnode/tests/msp430f5529-test.py"
}

// Add comments here.

node("rpi1") {
  sh 'echo "rpi1 node test."'
  
}
