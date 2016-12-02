node("raspi") {
  sh 'echo "THIS IS THE BUILD"'
  git url: 'https://github.com/kubostech/kubos.git'
  print env.JOB_NAME.replaceFirst('.+/', '')
  sh 'git checkout ' + env.JOB_NAME.replaceFirst('.+/', '')
  def workspace = pwd()
  sh "python ${workspace}/test/integration/test_raspi.py"
}
