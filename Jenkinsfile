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
                sh 'python3 python_files/test.py'
                junit 'python_files/results/*.xml'
            }
        }
    }
}