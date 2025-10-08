use core::panic;
use std::{env::{self, set_current_dir}, fs, path::Path, process::Command};

use serde::{Serialize};
use bincode;
#[derive(serde::Serialize, Debug)]
pub struct Language{
    languageName:String,
    assignmentName:String,
    moduleName:String,
    path:String,
    command:String,
    is_done:bool,
}
#[derive(Serialize, Debug)]
pub struct Languages{
    rust_db:Vec<Language>,
    go_db:Vec<Language>,
    java_db:Vec<Language>,
}
impl Languages{
   fn new() -> Self{
    Languages{
        rust_db : Vec::<Language>::new(),
        go_db : Vec::<Language>::new(),
        java_db : Vec::<Language>::new(),
    }
   }
fn add(&mut self, vector: Vec<Language>, lang: String) {
    match lang.as_str() {
        "rust" => self.rust_db = vector,
        "go" => self.go_db = vector,
        "java" => self.java_db = vector,
        _ => (),
    }
}
}
enum CommandType {
    Rust,
    Go,
    Java,
}

impl CommandType {
    fn get_command(&self) -> &'static str {
        match self {
            CommandType::Rust => "cargo test",
            CommandType::Go => "go test",
            CommandType::Java => "javac Main.java && java Main",
        }
    }
}
pub fn git_clone_exercises() -> Result<bool, String> {
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

            //Check if folder already exists
            if let Ok(direntry) = fs::read_dir("praxis"){
               println!("Repo already exists... Going to delete it");
               fs::remove_dir_all("praxis").expect("Failed to delete the repo");
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
                .args(&["sparse-checkout", "set", &format!("practicals")])
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
                    "Failed to clone practicals to exercises: {}",
                     stderr_msg
                ));
            }

            println!(
                "Successfully cloned the practicals",
                
            );
            Ok(true)
        }

pub fn get_modules(lang: &str) -> Result<Vec<Language>, String> {
    let path = format!("practicals/{}", lang);
    let mut modules = Vec::new();

    if let Ok(directories) = fs::read_dir(&path) {
        for entry in directories {
            if let Ok(module) = entry {
                if let Ok(module_type) = module.file_type() {
                    if module_type.is_dir() {
                        let module_name = module.file_name().into_string().unwrap_or_default();
                         let path = format!("practicals/{}/{}", lang,module_name);
                        if let Ok(assignments) = fs::read_dir(path){
                            for assignment in assignments{
                                if let Ok(assignment_name) = assignment{
                                    let filetype = assignment_name.file_type().unwrap();
                                    if(filetype.is_dir()){
                                        let content = Language{
                                            moduleName:module_name.clone(),
                                            assignmentName:assignment_name.file_name().to_string_lossy().to_string(),
                                            is_done:false,
                                            path:format!("{}/{}",module_name,assignment_name.file_name().to_string_lossy().to_string()),
                                            command:match lang {
                                                "rust" => CommandType::Rust.get_command().to_string(),
                                                "go" => CommandType::Go.get_command().to_string(),
                                                "java" => CommandType::Java.get_command().to_string(),
                                                _ => "".to_string(),
                                            },
                                            languageName : lang.to_string()
                                        };
                                        modules.push(content);
                                    }

                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(modules)
    } else {
        Err(format!("Failed to read directory: {}", path))
    }
}
pub fn update_dbs() -> Result<Languages, String> {
    let mut languages = Languages::new();
    if let Ok(directories) = fs::read_dir("practicals/") {
        for entries in directories {
            if let Ok(dir) = entries {
                if let Ok(file) = dir.file_type() {
                    if file.is_dir() && dir.file_name() == "rust" {
                        let rust_content = get_modules("rust").unwrap();
                        languages.add(rust_content, "rust".to_string());
                    }
                    if file.is_dir() && dir.file_name() == "go" {
                        let go_content = get_modules("go").unwrap();
                        languages.add(go_content, "go".to_string())
                    }
                    if file.is_dir() && dir.file_name() == "java" {
                        let go_content = get_modules("java").unwrap();
                        languages.add(go_content, "java".to_string())
                    }
                }
            }
        }
        println!("{:?}",languages);
        Ok(languages)
    } else {
        Err("Failed to read directory: practicals/".to_string())
    }
}



pub fn serializer(languages:Languages)->bincode::Result<()>{
let encoded: Vec<u8> = bincode::serialize(&languages)?;
std::fs::write("db.bin", &encoded).expect("write failed");
println!("Created db.bin in current directory");
Ok(())
}