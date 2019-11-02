pipeline {
    agent {
        docker { image 'python:3.8.0' }
    }
    stages {
        stage('Build') {
            steps {
                sh 'pip install -r requirements.txt'
            }
        }
        stage('Python Unit Tests') {
            steps {
                sh 'python3 -m unittest discover python -p "*_tests.py"'
            }
            post {
                always {
                  junit 'test-reports/*.xml'
                }
            }
        }
    }
}