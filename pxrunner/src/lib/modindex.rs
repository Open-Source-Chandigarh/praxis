//Todo custom error type and error handling for enum.
#[allow(non_snake_case)]
pub mod modernindx {
    use clap::builder::ValueParserFactory;
    use core::fmt;
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;
    use std::collections::HashMap;
    use std::env::{current_dir, var};
    use std::fs::{DirEntry, FileType};
    use std::str::FromStr;
    use std::{io, path};
    use walkdir::WalkDir;

    #[derive(Deserialize, Serialize, Debug, Default)]
    #[warn(dead_code)]
    pub struct Exercise {
        //purpose: meta data about assignments.
        pub name: String,
        pub passed: bool,
        pub language: String,
        pub parentmodule: String,
        pub path: String,
    }

    impl fmt::Display for Exercise {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Exercise '{}' [{}]\n  Module: {}\n  Path: {}\n  Status: {}",
                self.name,
                self.language,
                self.parentmodule,
                self.path,
                if self.passed {
                    "✓ PASSED"
                } else {
                    "✗ NOT PASSED"
                }
            )
        }
    }

    #[derive(Debug, Default)]
    pub struct SkillCourses {
        //purpose: localdb
        pub courseName: String,
        pub questions: Vec<Exercise>,
    }

    // #[derive(Debug, Default)]
    // pub struct JavaCourses {
    //     //purpose: localdb
    //     pub questions: Vec<Exercise>,
    // }
    //
    // #[derive(Debug, Default)]
    // pub struct GoCourses {
    //     //purpose: localdb
    //     pub questions: Vec<Exercise>,
    // }

    #[allow(non_camel_case_types)]
    // enum NeededDirs {
    //     practicals,
    //     praxis,
    //     rust,
    //     go,
    //     java,
    // }
    //
    // impl fmt::Display for NeededDirs {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         match self {
    //             NeededDirs::practicals => write!(f, "practicals"),
    //             NeededDirs::java => write!(f, "java"),
    //             NeededDirs::rust => write!(f, "rust"),
    //             NeededDirs::go => write!(f, "go"),
    //             NeededDirs::praxis => write!(f, "praxis"),
    //         }
    //     }
    // }

    // impl FromStr for NeededDirs {
    //     type Err = String;
    //     fn from_str(s: &str) -> Result<Self, Self::Err> {
    //         match s.to_lowercase().as_str() {
    //             "java" => Ok(NeededDirs::java),
    //             "rust" => Ok(NeededDirs::rust),
    //             "go" => Ok(NeededDirs::go),
    //             "practicals" => Ok(NeededDirs::practicals),
    //             "praxis" => Ok(NeededDirs::praxis),
    //             _ => Err("Irrelevant Directory".to_string()),
    //         }
    //     }
    // }

    pub fn checkWorkDir() -> Result<bool, String> {
        let mypath = current_dir().unwrap().to_string_lossy().to_string();
        let pathvec = mypath.split('/').collect::<Vec<&str>>();
        let mut inside_praxis = false;
        let mut inside_practicals = false;
        let mut inside_exercises = false;
        for i in pathvec.iter() {
            if *i == "praxis" {
                inside_praxis = true;
            }
            if *i == "practicals" {
                inside_practicals = true;
            }
            if *i == "java" || *i == "rust" || *i == "go" {
                inside_exercises = true;
            }
        }
        if inside_exercises && inside_practicals && inside_praxis {
            println!("{}", mypath);
            Ok(true)
        } else {
            println!("{}", mypath);
            Err("Not in the working directory".to_string())
        }
    }

    fn find_dir(start_path: &str, target_name: &str) -> Option<String> {
        for entry in WalkDir::new(start_path).max_depth(8) {
            if let Ok(entry) = entry {
                if entry.file_type().is_dir() && entry.file_name() == target_name {
                    return entry.path().to_str().map(|s| s.to_string());
                }
            }
        }
        None
    }

    pub fn trywalk() -> io::Result<Vec<String>> {
        let mut resvec: Vec<String> = Vec::new();
        let home_dir = var("HOME").unwrap_or_else(|_| ".".to_string());
        let start_path = find_dir(&home_dir, "praxis").unwrap_or_else(|| ".".to_string());

        for entry in WalkDir::new(start_path) {
            match entry {
                Ok(x) => {
                    if x.file_type().is_dir() {
                        let filepath = x.path();
                        let filestr = filepath.to_str().unwrap();
                        resvec.push(filestr.to_string());
                    }
                }
                Err(y) => resvec.push(format!("Error Parsing the directory : {}", y)),
            }
        }
        Ok(resvec)
    }

    // Todo Implement what the errors might be.
    pub fn filterTrywalk(pathvec: Vec<String>) -> io::Result<Vec<String>> {
        let mut filertedExercisesPaths: Vec<String> = Vec::new();
        for i in pathvec.iter() {
            if i.contains("src") && i.contains("practicals") {
                let pathlenWithoutSrc = i.len() - 3;
                if let Some(slice) = i.get(0..pathlenWithoutSrc) {
                    filertedExercisesPaths.push(slice.to_string());
                }
            }
        }
        Ok(filertedExercisesPaths)
    }

    pub fn findLang_inPath(pathvec: &Vec<String>) -> Option<String> {
        for subpath in pathvec.iter() {
            if subpath == "rust" {
                return Some("rust".to_string());
            } else if subpath == "go" {
                return Some("go".to_string());
            } else if subpath == "java" {
                return Some("java".to_string());
            }
        }
        None
    }

    //purpose: fills the localdb with exercises across the whole practical
    // Todo Implement what the errors might be and how to handle it.
    // Todo for windows the split character is "\" not "/" which is in linux.
    pub fn fillExercises(_pathvec: Vec<String>) -> io::Result<Vec<SkillCourses>> {
        let mut exerciseVec: Vec<SkillCourses> = Vec::new();

        let rustCourse = SkillCourses {
            courseName: "rust".to_string(),
            questions: Vec::new(),
        };

        let goCourse = SkillCourses {
            courseName: "go".to_string(),
            questions: Vec::new(),
        };

        let javaCourse = SkillCourses {
            courseName: "java".to_string(),
            questions: Vec::new(),
        };

        exerciseVec.push(rustCourse);
        exerciseVec.push(goCourse);
        exerciseVec.push(javaCourse);

        if let Ok(paths) = trywalk() {
            if let Ok(filteredPaths) = filterTrywalk(paths) {
                for Exercisepath in filteredPaths.into_iter() {
                    let patharray = Exercisepath.split("/").map(|e| e.to_string()).collect();
                    let lang = findLang_inPath(&patharray)
                        .unwrap_or_else(|| "Language for the exercise not found".to_string())
                        .to_string();
                    let exercise = Exercise {
                        name: patharray[patharray.len() - 2].to_owned(),
                        passed: false,
                        language: lang.clone(),
                        parentmodule: patharray[patharray.len() - 3].to_owned(),
                        path: Exercisepath,
                    };

                    if lang == "rust".to_string() {
                        exerciseVec[0].questions.push(exercise);
                    } else if lang == "go".to_string() {
                        exerciseVec[1].questions.push(exercise);
                    } else if lang == "java".to_string() {
                        exerciseVec[2].questions.push(exercise);
                    }
                }
            }
        }
        Ok(exerciseVec)
    }

    // let myinfo = Details {
    //     name: "Fanibhushan".to_string(),
    //     age: 20,
    //     married: false,
    // };
    // let myinfo_string = serde_json::to_string(&myinfo).unwrap();
    // let myinfo_str = myinfo_string.as_str();
    // println!("{}", myinfo_str);
    // let myinfo_struct: Details = serde_json::from_str(myinfo_str).unwrap();
    // println!("{:?}", myinfo_struct);
}

#[cfg(test)]
mod tests {
    use super::modernindx::*;

    #[test]
    fn test_filterWalk() {
        println!("\n=== Testing filterTrywalk() ===");
        match trywalk() {
            Ok(unfilteredRes) => match filterTrywalk(unfilteredRes) {
                Ok(result) => {
                    println!("Found {} filtered paths", result.len());
                    for i in result.iter() {
                        println!("{}", i);
                    }
                }
                Err(e) => println!("Error in filterTrywalk: {}", e),
            },
            Err(e) => println!("Error in trywalk: {}", e),
        }
    }

    #[test]
    fn test_check_work_dir() {
        println!("\n=== Testing checkWorkDir() ===");
        match checkWorkDir() {
            Ok(result) => {
                println!("✓ checkWorkDir returned: {}", result);
                println!("Current directory is valid");
            }
            Err(e) => {
                println!("✗ checkWorkDir error: {}", e);
                println!("Note: This is expected if not in praxis/practicals/[lang] directory");
            }
        }
    }

    #[test]
    fn test_trywalk() {
        println!("\n=== Testing trywalk() ===");
        match trywalk() {
            Ok(result) => {
                println!("✓ trywalk completed successfully");
                println!("Found {} directories", result.len());
                println!("First few directories:");
                for (i, dir) in result.iter().take(10).enumerate() {
                    println!("  {}: {}", i + 1, dir);
                }
            }
            Err(e) => {
                println!("✗ trywalk error: {}", e);
            }
        }
    }

    #[test]
    fn test_exercise_struct() {
        println!("\n=== Testing Exercise struct ===");
        let exercise = Exercise {
            name: "Test Exercise".to_string(),
            passed: true,
            language: "rust".to_string(),
            parentmodule: "basics".to_string(),
            path: "/path/to/exercise".to_string(),
        };
        println!("Created Exercise: {:?}", exercise);
        println!("Exercise name: {}", exercise.name);
        println!("Exercise passed: {}", exercise.passed);
        println!("Exercise language: {}", exercise.language);
    }

    #[test]
    fn test_exercise_default() {
        println!("\n=== Testing Exercise::default() ===");
        let default_exercise = Exercise::default();
        println!("Default Exercise: {:?}", default_exercise);
        assert_eq!(default_exercise.name, "");
        assert_eq!(default_exercise.passed, false);
    }

    #[test]
    fn test_exercise_serialization() {
        println!("\n=== Testing Exercise serialization ===");
        let exercise = Exercise {
            name: "Serialization Test".to_string(),
            passed: true,
            language: "rust".to_string(),
            parentmodule: "serde_module".to_string(),
            path: "/test/path".to_string(),
        };

        // Serialize to JSON
        let json_string = serde_json::to_string(&exercise).unwrap();
        println!("Serialized JSON: {}", json_string);

        // Deserialize back
        let deserialized: Exercise = serde_json::from_str(&json_string).unwrap();
        println!("Deserialized Exercise: {:?}", deserialized);

        assert_eq!(exercise.name, deserialized.name);
        assert_eq!(exercise.passed, deserialized.passed);
    }

    #[test]
    fn test_fill_exercises() {
        println!("\n=== Testing fillExercises() ===");
        match fillExercises(vec![]) {
            Ok(courses) => {
                // println!("✓ fillExercises returned {} exercises", exercises.len());
                // println!("\nFirst 5 exercises:");
                // for (i, ex) in exercises.iter().enumerate() {
                //     println!("\n  Exercise {}:", i + 1);
                //     println!("    Name: {}", ex.name);
                //     println!("    Language: {}", ex.language);
                //     println!("    Parent Module: {}", ex.parentmodule);
                //     println!("    Path: {}", ex.path);
                //     println!("    Passed: {}", ex.passed);
                //     println!("    Full struct: {:?}", ex);
                // }
                for i in courses.into_iter() {
                    for (j, ex) in i.questions.iter().enumerate() {
                        println!("\n  Exercise {}:", j + 1);
                        println!("    Name: {}", ex.name);
                        println!("    Language: {}", ex.language);
                        println!("    Parent Module: {}", ex.parentmodule);
                        println!("    Path: {}", ex.path);
                        println!("    Passed: {}", ex.passed);
                        println!("    Full struct: {:?}", ex);
                    }
                }
            }

            Err(e) => {
                println!("✗ fillExercises error: {}", e);
            }
        }
    }
}
