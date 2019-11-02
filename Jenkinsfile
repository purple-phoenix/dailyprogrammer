pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh 'pip install --user -r python_files/requirements.txt'
            }
        }
        stage('Python Unit Tests') {
            steps {
                sh 'python3 -m unittest discover python_files -p "*_tests.py"'
            }
            post {
                always {
                  junit 'test-reports/*.xml'
                }
            }
        }
    }
}