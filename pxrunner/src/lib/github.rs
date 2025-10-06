// All commands for Running and installing Books
//Todo Have to do unit tests on each of these functionalities that they validate the expectations
#![allow(non_camel_case_types)]

pub mod cli {
    use clap::Parser;
    use std::fmt;
    use std::str::FromStr;

    //command line metadata
    #[derive(Debug, Parser)]
    #[command(
        version = "0.0.1",
        about,
        long_about = "Exercises test runner for praxis. Turn theory into practice!!"
    )]
    pub struct Args {
        #[clap(value_parser)]
        pub values: Vec<CliArgs>,
    }

    // available options for language selection
    #[derive(Debug, Clone)]
    pub enum Langs {
        go,
        java,
        rust,
    }

    // all the available system commands for px
    #[derive(Debug, Clone)]
    pub enum Pxcommands {
        init,
        run,
    }

    //The cliArgs will only have two variants , either system commands or the language ones.
    #[derive(Debug, Clone)]
    pub enum CliArgs {
        lang_option(Langs),
        commands_option(Pxcommands),
    }

    //impl of Display in our custom enums ,
    //so that they can be displayed in the format!
    impl fmt::Display for CliArgs {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CliArgs::commands_option(cmd) => match cmd {
                    Pxcommands::init => write!(f, "init"),
                    Pxcommands::run => write!(f, "run"),
                },
                CliArgs::lang_option(lang) => match lang {
                    Langs::go => write!(f, "go"),
                    Langs::java => write!(f, "java"),
                    Langs::rust => write!(f, "rust"),
                },
            }
        }
    }

    //impl of Display in our custom enums ,
    //so that they can be displayed in the format!
    impl fmt::Display for Langs {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Langs::go => write!(f, "go"),
                Langs::java => write!(f, "java"),
                Langs::rust => write!(f, "rust"),
            }
        }
    }

    //impl of Display in our custom enums ,
    //so that they can be displayed in the format!
    impl fmt::Display for Pxcommands {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Pxcommands::init => write!(f, "init"),
                Pxcommands::run => write!(f, "run"),
            }
        }
    }

    //impl of FromStr for our Cliargs so that we can
    //convert the values gotten through command line into respective enum.
    impl FromStr for CliArgs {
        type Err = String;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match input.to_lowercase().as_str() {
                "rust" => Ok(CliArgs::lang_option(Langs::rust)),
                "go" => Ok(CliArgs::lang_option(Langs::go)),
                "init" => Ok(CliArgs::commands_option(Pxcommands::init)),
                "java" => Ok(CliArgs::lang_option(Langs::java)),
                "" => Ok(CliArgs::commands_option(Pxcommands::run)),
                _ => Err(format!("`{}` is not a valid argument", input)),
            }
        }
    }
}

pub mod github_traits {
    use super::cli::{Langs, Pxcommands};

    //whenever we have to setup the exercises we require 2 things ,
    //get it from github and get its dependencies which are required to
    //run them.
    // both this functions later get called in the install exercises methods
    //sequentially to both clone the exercises as well as get its dependencies.
    pub trait ExercisesSetup {
        fn git_clone_exercises(lang: &Langs) -> Result<bool, String>;
        //here install dependencies will match for Lang enum variants and will
        //apply respective installation for language dependencies.
        fn install_dependencies(lang: &Langs) -> Result<bool, String>;
    }

    //contains the function which can install dependencies langauge specific.
    //this methods later get used in the install dependencies method later.
    //Todo implement installation for java and go
    //Todo The implementation should be platform agnostic.
    pub trait LanguageDependencies {
        fn install_rust() -> Result<bool, String>;
        // fn install_java() -> Result<bool, String>;
        // fn install_go() -> Result<bool, String>;
    }

    //this trait has all the methods which can be called directly whenever its required
    //to install exercises and setup the working environment for a language.
    pub trait InstallExercises {
        //Todo this functions will run the installation of exercises function , followed by the checking and
        //Todo installation of respective dependencies function.
        fn rust() -> Result<bool, String>;
        // fn go() -> Result<bool, String>;
        // fn java()-> Result<bool, String>;
    }

    pub trait SystemCommands {
        fn init() -> Result<bool, String>;
        fn run() -> Result<bool, String>;
    }
}

pub mod github_implementations {
    use super::cli::{Langs, Pxcommands};
    use super::github_traits::{
        ExercisesSetup, InstallExercises, LanguageDependencies, SystemCommands,
    };
    use std::env::set_current_dir;
    use std::path::Path;
    use std::process::Command;

    impl SystemCommands for Pxcommands {
        //Todo This should be able to clone the whole repo without any sparse checkout.
        //Todo this should be able to check all the dependencies for all the repo.
        fn init() -> Result<bool, String> {
            // Check if git is installed
            let mut git_check_cmd;
            if cfg!(target_os = "windows") {
                let mut cmd = Command::new("where");
                cmd.arg("git");
                git_check_cmd = cmd;
            } else {
                let mut cmd = Command::new("which");
                cmd.arg("git");
                git_check_cmd = cmd;
            };

            let gitinstalled = git_check_cmd
                .output()
                .map_err(|e| format!("Failed to check git installation: {}", e))?;

            if gitinstalled.status.success() {
                println!("✅ Git is installed");
            } else {
                let stderr_msg = String::from_utf8_lossy(&gitinstalled.stderr);
                return Err(format!("❌ Git not found.\n{}", stderr_msg));
            }

            // Clone the repository
            let clone_go = Command::new("git")
                .arg("clone")
                .arg("--filter=blob:none")
                .arg("--no-checkout")
                .arg("https://github.com/Open-Source-Chandigarh/praxis.git")
                .output()
                .map_err(|e| format!("Failed to execute 'git clone': {}", e))?;

            if !clone_go.status.success() {
                let stderr_msg = String::from_utf8_lossy(&clone_go.stderr);
                return Err(format!("Failed to clone the repo.\n{}", stderr_msg));
            }

            println!("✅ Repository cloned successfully.");
            let praxis_path = Path::new("praxis");
            set_current_dir(&praxis_path)
                .map_err(|e| format!("Failed to change directory: {}", e))?;
            println!(
                "Successfully changed working directory to {}!",
                praxis_path.display()
            );

            // Rest of implementation with sparse checkout
            let sparse_checkout = Command::new("git")
                .args(&["sparse-checkout", "init", "--cone"])
                .output()
                .map_err(|e| format!("Failed to execute git sparse-checkout init --cone: {}", e))?;

            if !sparse_checkout.status.success() {
                let stderr_msg = String::from_utf8_lossy(&sparse_checkout.stderr);
                return Err(format!("Failed to do sparse checkout: {}", stderr_msg));
            }

            println!("Executed sparse checkout command successfully");
            let choose_checkout = Command::new("git")
                .args(&["sparse-checkout", "set", "practicals/rust"])
                .output()
                .map_err(|e| format!("Failed to execute sparse-checkout set: {}", e))?;

            if !choose_checkout.status.success() {
                let stderr_msg = String::from_utf8_lossy(&choose_checkout.stderr);
                return Err(format!("Failed to set sparse checkout: {}", stderr_msg));
            }

            println!("Chose checkout folders successfully");

            let git_checkout = Command::new("git")
                .args(&["checkout", "main"])
                .output()
                .map_err(|e| format!("Failed to execute git checkout main: {}", e))?;

            if !git_checkout.status.success() {
                let stderr_msg = String::from_utf8_lossy(&git_checkout.stderr);
                return Err(format!("Failed to clone the exercises: {}", stderr_msg));
            }

            println!("Successfully cloned the exercises from practicals");

            //----Check for dependencies of all the exercises----
            //Fix: error propagation done instead of using expect
            Langs::install_dependencies(&Langs::rust)?;
            Ok(true)
        }

        //Todo most important part, should be able to run tests , and evaluate upon them.
        fn run() -> Result<bool, String> {
            //Todo to be implemented later.
            Ok(false)
        }
    }

    impl InstallExercises for Langs {
        fn rust() -> Result<bool, String> {
            // Fix: Propagated errors instead of using expect
            let clone_rust_repo = Self::git_clone_exercises(&Langs::rust)?;
            let check_rustup = Self::install_dependencies(&Langs::rust)?;

            if check_rustup && clone_rust_repo {
                Ok(true)
            } else {
                Err("Failed to complete rust setup".to_string())
            }
        }
    }

    //here resides only the implementation of install_rust
    //which installs rustup for linux devices only.
    impl LanguageDependencies for Langs {
        // Todo Add the code for downloading rustup in windows
        fn install_rust() -> Result<bool, String> {
            // --- 1. CHECK FOR RUSTUP ---
            let output = Command::new("rustup")
                .arg("--version")
                .output()
                .map_err(|e| format!("Failed to execute 'rustup': {}", e))?;

            if output.status.success() {
                println!("✅ rustup is already installed.");
                return Ok(true);
            }

            // --- 2. INSTALL RUSTUP (If check failed) ---
            println!("❌ rustup not found. Attempting installation...");

            let install_output = if cfg!(target_os = "windows") {
                return Err("Windows rustup installation not implemented yet".to_string());
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
                    .output()
                    .map_err(|e| format!("Failed to start rustup installation script: {}", e))?
            };

            // --- 3. CHECK INSTALLATION RESULT ---
            if install_output.status.success() {
                println!("✅ rustup installation successful.");
                // Fix: Proper error handling for UTF-8 conversion
                let stdout = String::from_utf8_lossy(&install_output.stdout);
                println!("Installer Output:\n{}", stdout);
                Ok(true)
            } else {
                let stderr = String::from_utf8_lossy(&install_output.stderr);
                eprintln!("❌ rustup installation failed.");
                Err(format!("Installer stderr:\n{}", stderr))
            }
        }
    }

    //here resides the implementations for methods which will be setup
    //exercises and running requirements for a language.
    impl ExercisesSetup for Langs {
        fn git_clone_exercises(lang: &Langs) -> Result<bool, String> {
            // Check if git is installed
            let mut git_check_cmd;
            if cfg!(target_os = "windows") {
                let mut cmd = Command::new("where");
                cmd.arg("git");
                git_check_cmd = cmd;
            } else {
                let mut cmd = Command::new("which");
                cmd.arg("git");
                git_check_cmd = cmd;
            };

            let gitinstalled = git_check_cmd
                .output()
                .map_err(|e| format!("Failed to check git installation: {}", e))?;

            if gitinstalled.status.success() {
                println!("✅ Git is installed");
            } else {
                let stderr_msg = String::from_utf8_lossy(&gitinstalled.stderr);
                return Err(format!("❌ Git not found.\n{}", stderr_msg));
            }

            // Clone the repository
            let clone_go = Command::new("git")
                .arg("clone")
                .arg("--filter=blob:none")
                .arg("--no-checkout")
                .arg("https://github.com/Open-Source-Chandigarh/praxis.git")
                .output()
                .map_err(|e| format!("Failed to execute 'git clone': {}", e))?;

            if !clone_go.status.success() {
                let stderr_msg = String::from_utf8_lossy(&clone_go.stderr);
                return Err(format!("Failed to clone the repo.\n{}", stderr_msg));
            }

            println!("✅ Repository cloned successfully.");
            let praxis_path = Path::new("praxis");
            set_current_dir(&praxis_path)
                .map_err(|e| format!("Failed to change directory: {}", e))?;
            println!(
                "Successfully changed working directory to {}!",
                praxis_path.display()
            );

            // Rest of implementation with sparse checkout
            let sparse_checkout = Command::new("git")
                .args(&["sparse-checkout", "init", "--cone"])
                .output()
                .map_err(|e| format!("Failed to execute git sparse-checkout init --cone: {}", e))?;

            if !sparse_checkout.status.success() {
                let stderr_msg = String::from_utf8_lossy(&sparse_checkout.stderr);
                return Err(format!("Failed to do sparse checkout: {}", stderr_msg));
            }

            println!("Executed sparse checkout command successfully");
            let choose_checkout = Command::new("git")
                .args(&["sparse-checkout", "set", &format!("practicals/{}", lang)])
                .output()
                .map_err(|e| format!("Failed to execute sparse-checkout set: {}", e))?;

            if !choose_checkout.status.success() {
                let stderr_msg = String::from_utf8_lossy(&choose_checkout.stderr);
                return Err(format!("Failed to set sparse checkout: {}", stderr_msg));
            }

            println!("Chose checkout folders successfully");

            let git_checkout = Command::new("git")
                .args(&["checkout", "main"])
                .output()
                .map_err(|e| format!("Failed to execute git checkout main: {}", e))?;

            if !git_checkout.status.success() {
                let stderr_msg = String::from_utf8_lossy(&git_checkout.stderr);
                return Err(format!(
                    "Failed to clone to {} exercises: {}",
                    lang, stderr_msg
                ));
            }

            println!(
                "Successfully cloned the {} exercises from practicals/{}",
                lang, lang
            );
            Ok(true)
        }

        fn install_dependencies(lang: &Langs) -> Result<bool, String> {
            match lang {
                Langs::rust => {
                    // Fix: Propagate error instead of using expect
                    Self::install_rust()?;
                    Ok(true)
                }
                _ => Err(format!(
                    "dependency installation for {} is not supported yet!!",
                    lang
                )),
            }
        }
    }
}

pub mod app_logic {
    use super::cli::{Args, CliArgs, Langs, Pxcommands};
    use super::github_traits::{InstallExercises, SystemCommands};
    use clap::Parser;

    pub fn run() {
        let cli = Args::parse();
        for arg in &cli.values {
            match arg {
                CliArgs::lang_option(Langs::rust) => match <Langs as InstallExercises>::rust() {
                    Ok(_) => println!("Rust exercises setup complete."),
                    Err(e) => eprintln!("Error: {}", e),
                },
                CliArgs::lang_option(lang) => {
                    println!("Setup for {} not yet implemented.", lang);
                }
                CliArgs::commands_option(Pxcommands::init) => {
                    match <Pxcommands as SystemCommands>::init() {
                        Ok(_) => println!("Init command executed successfully."),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                CliArgs::commands_option(Pxcommands::run) => {
                    println!("Run command not yet implemented.");
                }
            }
        }
    }
}

//main function implementation.
fn main() {
    app_logic::run();
}
