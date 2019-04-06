pipeline {
    agent none
    stages {
        stage('Cargo Fmt') {
            environment {
                CARGO_HOME = '/home/jenkins/.cargo'
                RUSTUP_HOME = '/home/jenkins/.rustup'
                RUSTFLAGS = "-D warnings"
            }
            agent {
		docker {
		    image 'magnonellie/amethyst-dependencies:stable'
		    label 'docker'
		} 
            }
            steps {
                echo 'Checking formatting...'
                sh '$CARGO_HOME/bin/cargo fmt -- --check'
            }
        }
        stage('Cargo Check') {
            environment {
                CARGO_HOME = '/home/jenkins/.cargo'
                RUSTUP_HOME = '/home/jenkins/.rustup'
                RUSTFLAGS = "-D warnings"
            }
            agent {
		docker {
		    image 'magnonellie/amethyst-dependencies:stable'
		    label 'docker'
		} 
            }
            steps {
                echo 'Running Cargo check...'
                sh '$CARGO_HOME/bin/cargo check --all --all-features --all-targets'
            }
        }
        stage('Run Tests') {
            parallel {
                stage("Test on Windows") {                    
                    environment {
                        CARGO_HOME = 'C:\\Users\\root\\.cargo'
                        RUSTUP_HOME = 'C:\\Users\\root\\.rustup'
                    }
                    agent { 
                        label 'windows' 
                    }
                    steps {
                        echo 'Beginning tests...'
                        bat 'C:\\Users\\root\\.cargo\\bin\\cargo test --all'
                        echo 'Tests done!'
                    }
                }
                stage("Test on Linux") {
                    environment {
                        CARGO_HOME = '/home/jenkins/.cargo'
                        RUSTUP_HOME = '/home/jenkins/.rustup'
                    }
                    agent {
			docker {
			    image 'magnonellie/amethyst-dependencies:stable'
			    label 'docker'
			} 
                    }
                    steps {
                        echo 'Beginning tests...'
                        sh '/home/jenkins/.cargo/bin/cargo test --all"'
                        echo 'Tests done!'
                    }
                }
                stage("Test on macOS") {
                    environment {
                        CARGO_HOME = '/Users/jenkins/.cargo'
                        RUSTUP_HOME = '/Users/jenkins/.rustup'
                    }
                    agent {
                        label 'mac'
                    }
                    steps {
                        echo 'Beginning tests...'
                        sh '/Users/jenkins/.cargo/bin/cargo test'
                        sh '/Users/jenkins/.cargo/bin/cargo test --all'
                        echo 'Tests done!'
                    }
                }
            }
        }
        // stage('Calculate Coverage') {
        //     environment {
        //         CARGO_HOME = '/home/jenkins/.cargo'
        //         RUSTUP_HOME = '/home/jenkins/.rustup'
        //         RUSTFLAGS = "-D warnings"
        //     }
        //     agent {
        //         label 'linux'
        //     }
        //     steps {
        //         withCredentials([string(credentialsId: 'codecov_token', variable: 'CODECOV_TOKEN')]) {
        //             echo 'Calculating code coverage...'
        //             sh 'for file in target/debug/amethyst-[a-f0-9]*[^\\.d]; do mkdir -p \"target/cov/$(basename $file)\"; kcov --exclude-pattern=/.cargo,/usr/lib --verify \"target/cov/$(basename $file)\" \"$file\"; done'
        //             echo "Uploading coverage..."
        //             sh "curl -s https://codecov.io/bash | bash -s - -t $CODECOV_TOKEN"
        //             echo "Uploaded code coverage!"
        //         }
        //     }
        // }
    }
}
