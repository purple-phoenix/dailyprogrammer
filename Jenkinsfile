pipeline {
    agent any
    environment {
        PATH = "/var/jenkins_home/.cargo/bin:$PATH"
    }
    stages {
        stage('Build Python') {
            steps {
                sh 'python3 -m pip install --user -r python_files/requirements.txt'
            }
        }
        stage('Python Unit Tests') {
            steps {
                sh 'python3 python_files/test.py'
            }
        }

        stage('Build and Test Rust') {
            dir('rust_files') {
                sh 'cargo test'
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

