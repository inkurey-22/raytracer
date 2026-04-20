pipeline {
  agent any

  environment {
    CONTAINER = 'rust:trixie'
    MIRROR_REPO = 'https://github.com/EpitechPGE2-2025/G-OOP-400-LIL-4-1-raytracer-1.git'
    MIRROR_CRED = 'jenkins-pat'
  }

  stages {
    stage('Checkout') {
      steps { checkout scm }
    }

    stage('Build') {
      steps {
        script {
          docker.image(env.CONTAINER).inside('-i') {
            try {
              sh label: 'cargo build', script: 'cargo build'
            } catch (err) {
              error('Build failed while running cargo build')
            }
          }
        }
      }
    }

    stage('Test') {
      steps {
        script {
          docker.image(env.CONTAINER).inside('-i') {
            try {
              sh label: 'cargo test', script: 'cargo test'
            } catch (err) {
              error('Test failed while running cargo test')
            }
          }
        }
      }
    }

    stage('Clippy') {
      steps {
        script {
          docker.image(env.CONTAINER).inside('-i') {
            try {
              sh label: 'cargo clippy', script: 'cargo clippy --all-targets --all-features -- -D warnings'
            } catch (err) {
              error('Clippy failed while running cargo clippy --all-targets --all-features -- -D warnings')
            }
          }
        }
      }
    }

    stage('Mirror to repo') {
      when { expression { currentBuild.currentResult == 'SUCCESS' } }
      steps {
        withCredentials([usernamePassword(credentialsId: env.MIRROR_CRED, usernameVariable: 'GIT_USER', passwordVariable: 'GIT_TOKEN')]) {
          script {
            try {
              sh label: 'mirror to repo', script: '''bash -c "
set -e
git config user.name 'jenkins-bot'
git config user.email 'jenkins@inkurey.fr'
# Remove existing mirror remote if present
git remote remove mirror 2>/dev/null || true

# Add correct mirror remote with token
git remote add mirror https://$GIT_USER:$GIT_TOKEN@${MIRROR_REPO#https://}
git fetch mirror || true

# Push to the mirror repository
git push --force-with-lease mirror HEAD:refs/heads/main
"'''
            } catch (err) {
              error('Mirror failed while pushing to the base or target repository')
            }
          }
        }
      }
    }
  }

  post {
    failure { echo 'Build, test, or clippy checks failed — not mirroring.' }
    success { echo 'All done — mirrored to target repo.' }
  }
}
