pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh 'python3 -m pip install --user -r python_files/requirements.txt'
            }
        }
        stage('Python Unit Tests') {
            steps {
                sh 'python3 python_files/test.py'
            }
        }
    }
    post {
        always {
            junit 'results/*.xml'
            deleteDir()
        }
    }
}

