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
        parallel {

            stage('Python Unit Tests') {
            agent any
                steps {
                    sh 'python3 python_files/test.py'
                }
            }

            stage('Build and Test Rust') {
            agent any
                steps {
                    sh 'cargo test --manifest-path=rust_files/Cargo.toml'
                }
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

