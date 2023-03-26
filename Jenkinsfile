pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                timestamps {
                    echo 'Building GitHub Test Runner..'
                }
            }
        }
        stage('Test') {
            steps {
                timestamps {
                    echo 'Testing with GitHubTestRunner..'
                    sh 'cd client'
                    sh 'npm install'
                    sh 'npm run test'
                }
            }
        }
        stage('Deploy') {
            steps {
                timestamps {
                    echo 'Deploying with GitHubTestRunner..'
                }
            }
        }
    }
}
