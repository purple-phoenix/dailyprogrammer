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
        stage('Unit Tests') {
            parallel {

                stage('Python Unit Tests') {
                agent any
                    steps {
                        sh 'python3 python_files/test.py'
                        junit 'results/*.xml'
                        sh 'rm -rf results/*.xml'
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

    }
}

