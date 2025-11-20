//Todo custom error type and error handling for enum.
#[allow(non_snake_case)]
pub mod modernindx {
    use clap::builder::ValueParserFactory;
    use core::fmt;
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;
    use std::collections::HashMap;
    use std::env::{current_dir, var};
    use std::fs::{DirEntry, File, FileType};
    use std::path::Path;
    use std::str::FromStr;
    use std::{io, path};
    use walkdir::WalkDir;

    #[derive(Deserialize, Serialize, Debug, Default, Clone)]
    #[warn(dead_code)]
    pub struct Exercise {
        //purpose: meta data about assignments.
        pub name: String,
        pub passed: bool,
        pub language: String,
        pub parentmodule: String,
        pub path: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AppState {
        pub version: f32,
        pub startpath: String,
        pub db: Vec<SkillCourses>,
        pub Pathbindb: String,
    }

    // Purpose: Formats an Exercise for nice display output
    // Input: Reference to an Exercise object
    // Output: Formatted text showing exercise name, language, module, path, and pass/fail status
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

    #[derive(Deserialize, Serialize, Debug, Default, Clone)]
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

    // Purpose: Checks if you are in the correct working directory for exercises
    // Input: Nothing (reads current directory from system)
    // Output: Success (true) if inside praxis/practicals/[language] directory, Error message if not
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

    // Purpose: Searches for a directory by name within a starting path
    // Input: A starting path (text) and target directory name (text) to search for
    // Output: Full path to the directory if found, or None if not found (searches up to 8 levels deep)
    pub fn find_dir(start_path: &str, target_name: &str) -> Option<String> {
        for entry in WalkDir::new(start_path).max_depth(8) {
            if let Ok(entry) = entry {
                if entry.file_type().is_dir() && entry.file_name() == target_name {
                    return entry.path().to_str().map(|s| s.to_string());
                }
            }
        }
        None
    }

    //optimize it so that it only walks under practical folder and not everywhere else.
    // Purpose: Walks through all directories starting from the praxis folder
    // Input: Nothing (finds praxis folder automatically from your home directory)
    // Output: A list of all directory paths found inside praxis folder
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

    // Purpose: Filters directory paths to keep only exercise directories (removes /src from end)
    // Input: A list of directory paths (text)
    // Output: A filtered list containing only practical exercise paths without the /src suffix
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

    // Purpose: Detects which programming language a path belongs to
    // Input: A list of path segments (folder names from a full path)
    // Output: Language name ("rust", "go", or "java") if found in path, otherwise None
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

    // Purpose: Scans all exercise directories and creates a database of courses with their exercises
    // Input: A list of path strings (currently not used, function finds paths automatically)
    // Output: A list of courses (Rust, Go, Java) each containing their exercise information (name, path, language, module, status)
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

    // Purpose: Creates an AppState object with exercise database and configuration
    // Input: List of courses, version number (decimal), project path (text), database path (text)
    // Output: AppState object containing all the information, or None if creation fails
    pub fn makeAppdata(
        dbvec: Vec<SkillCourses>,
        ver: f32,
        projpath: String,
        dbpath: String,
    ) -> Option<AppState> {
        let appdata = AppState {
            version: ver,
            startpath: projpath,
            db: dbvec,
            Pathbindb: dbpath,
        };
        Some(appdata)
    }

    // Purpose: Checks if a file exists in a directory
    // Input: Directory path (text) and filename (text)
    // Output: True if file exists, false if not
    fn file_exists(dir: &str, file: &str) -> bool {
        Path::new(dir).join(file).exists()
    }

    // Purpose: Updates the application data file with current exercise information
    // Input: A list of courses with exercises
    // Output: Success if file is updated, Error if file operations fail
    pub fn updateVecData(data: Vec<SkillCourses>) -> io::Result<()> {
        let vecdb = data;
        let version = 0.5;
        let home_dir = var("HOME").unwrap_or_else(|_| ".".to_string());
        let start_path = find_dir(&home_dir, "praxis").unwrap_or_else(|| ".".to_string());
        let dbpath = start_path.clone() + "/practicals/";
        let appdatastruct = makeAppdata(vecdb, version, start_path.clone(), dbpath.clone())
            .expect("Failed to create the struct AppData");
        let filenamePath = dbpath.clone() + "data.bin";

        let db_file = File::create(&filenamePath)?;
        serde_cbor::to_writer(db_file, &appdatastruct)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(())
    }

    // Purpose: Reads saved application data from the data file
    // Input: Path to database directory (text)
    // Output: AppState object with version, paths, and exercise database, or Error if file doesn't exist or can't be read
    pub fn readAppdata(dbpath: &str) -> io::Result<AppState> {
        if !file_exists(dbpath, "data.bin") {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The file doesnt exist to read the data from",
            ));
        }
        let filepath = dbpath.to_string() + "data.bin";
        let db_file = File::open(&filepath)?;
        let appstatedata: AppState = serde_cbor::from_reader(db_file)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(appstatedata)
    }

    // Purpose: Initializes the database file with all exercises from the praxis folder
    // Input: Nothing (automatically scans all exercises and creates data.bin file)
    // Output: Success if database is created and initialized with exercises, Error if any operation fails
    pub fn initDB() -> io::Result<()> {
        let vecdb = fillExercises(vec![])?;
        updateVecData(vecdb)?;
        Ok(())
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
    use std::env::var;

    // Purpose: Tests if filtering of exercise paths works correctly
    // Input: Nothing (gets paths from trywalk function)
    // Output: Prints number of filtered exercise paths found and displays each path
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

    // Purpose: Tests if the current directory validation works
    // Input: Nothing (checks your current working directory)
    // Output: Success message if in correct directory, error message if not in praxis/practicals/language folder
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

    // Purpose: Tests if directory walking through praxis folder works
    // Input: Nothing (automatically finds and walks praxis directory)
    // Output: Prints total count of directories found and shows first 10 directory paths
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

    // Purpose: Tests if creating an Exercise object works correctly
    // Input: Creates a sample exercise with name, status, language, module, and path
    // Output: Prints the exercise details to verify all fields are stored properly
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

    // Purpose: Tests if creating an empty Exercise with default values works
    // Input: Nothing (uses default constructor)
    // Output: Creates Exercise with empty name, false passed status, and verifies these defaults
    #[test]
    fn test_exercise_default() {
        println!("\n=== Testing Exercise::default() ===");
        let default_exercise = Exercise::default();
        println!("Default Exercise: {:?}", default_exercise);
        assert_eq!(default_exercise.name, "");
        assert_eq!(default_exercise.passed, false);
    }

    // Purpose: Tests if converting Exercise to JSON and back works correctly
    // Input: Creates an Exercise object
    // Output: Converts it to JSON text, then converts JSON back to Exercise object, verifies data matches
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

    // Purpose: Tests if scanning and collecting all exercises into courses works
    // Input: Empty list (function finds exercises automatically)
    // Output: Displays all courses (Rust, Go, Java) with their exercises showing name, language, module, path, and status
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

    // Purpose: Tests if database initialization works correctly
    // Input: Nothing (creates data.bin file and fills it with all exercises)
    // Output: Prints success message if database is created with exercises, error message if it fails
    #[test]
    fn test_initDB() {
        println!("\n=== Testing initDB() ===");
        match initDB() {
            Ok(()) => {
                println!("✓ initDB completed successfully");
                println!("Database file created at praxis/practicals/data.bin");
                println!("All exercises have been indexed and saved");

                // Verify the database was created by trying to read it
                let home_dir = var("HOME").unwrap_or_else(|_| ".".to_string());
                if let Some(start_path) = find_dir(&home_dir, "praxis") {
                    let dbpath = format!("{}/practicals/", start_path);
                    println!("Attempting to read back the database from: {}", dbpath);

                    match readAppdata(&dbpath) {
                        Ok(appstate) => {
                            println!("✓ Successfully read back the database");
                            println!("  Version: {}", appstate.version);
                            println!("  Start path: {}", appstate.startpath);
                            println!("  Database path: {}", appstate.Pathbindb);
                            println!("  Number of courses: {}", appstate.db.len());

                            for course in appstate.db.iter() {
                                println!("\n  Course: {}", course.courseName);
                                println!("    Number of exercises: {}", course.questions.len());
                                if course.questions.len() > 0 {
                                    println!("    First exercise: {}", course.questions[0].name);
                                }
                            }
                        }
                        Err(e) => {
                            println!("✗ Failed to read back database: {}", e);
                        }
                    }
                } else {
                    println!("⚠ Could not find praxis directory to verify database");
                }
            }
            Err(e) => {
                println!("✗ initDB error: {}", e);
                println!(
                    "This may happen if praxis directory is not found or permissions are denied"
                );
            }
        }
    }

    // Purpose: Tests if updating database with new exercise data works
    // Input: Creates a sample list of courses with exercises
    // Output: Prints success if data is saved to file, error if operation fails
    #[test]
    fn test_updateVecData() {
        println!("\n=== Testing updateVecData() ===");

        // Create sample data
        let mut rust_course = SkillCourses {
            courseName: "rust".to_string(),
            questions: Vec::new(),
        };

        let sample_exercise = Exercise {
            name: "test_exercise".to_string(),
            passed: false,
            language: "rust".to_string(),
            parentmodule: "basics".to_string(),
            path: "/test/path/to/exercise".to_string(),
        };

        rust_course.questions.push(sample_exercise);

        let mut courses = vec![rust_course];

        // Add empty Go and Java courses
        courses.push(SkillCourses {
            courseName: "go".to_string(),
            questions: Vec::new(),
        });
        courses.push(SkillCourses {
            courseName: "java".to_string(),
            questions: Vec::new(),
        });

        println!("Created sample data with {} courses", courses.len());
        println!("  Rust course has {} exercises", courses[0].questions.len());

        match updateVecData(courses.clone()) {
            Ok(()) => {
                println!("✓ updateVecData completed successfully");
                println!("Sample data has been written to data.bin");

                // Try to read it back to verify
                let home_dir = var("HOME").unwrap_or_else(|_| ".".to_string());
                if let Some(start_path) = find_dir(&home_dir, "praxis") {
                    let dbpath = format!("{}/practicals/", start_path);

                    match readAppdata(&dbpath) {
                        Ok(appstate) => {
                            println!("✓ Successfully verified the update by reading back");
                            println!("  Number of courses in database: {}", appstate.db.len());

                            if appstate.db.len() > 0 && appstate.db[0].courseName == "rust" {
                                println!(
                                    "  Rust course exercises: {}",
                                    appstate.db[0].questions.len()
                                );
                                if appstate.db[0].questions.len() > 0 {
                                    println!(
                                        "  First exercise name: {}",
                                        appstate.db[0].questions[0].name
                                    );
                                    println!(
                                        "  First exercise language: {}",
                                        appstate.db[0].questions[0].language
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            println!("✗ Failed to verify update by reading: {}", e);
                        }
                    }
                } else {
                    println!("⚠ Could not find praxis directory to verify");
                }
            }
            Err(e) => {
                println!("✗ updateVecData error: {}", e);
                println!(
                    "This may happen if praxis directory is not found or file cannot be written"
                );
            }
        }
    }

    // Purpose: Tests if reading application data from file works correctly
    // Input: Database path where data.bin should exist
    // Output: Prints the database contents if read successfully, error if file doesn't exist or can't be read
    #[test]
    fn test_readAppdata() {
        println!("\n=== Testing readAppdata() ===");

        let home_dir = var("HOME").unwrap_or_else(|_| ".".to_string());

        if let Some(start_path) = find_dir(&home_dir, "praxis") {
            let dbpath = format!("{}/practicals/", start_path);
            println!("Looking for database at: {}", dbpath);

            match readAppdata(&dbpath) {
                Ok(appstate) => {
                    println!("✓ Successfully read database");
                    println!("\nDatabase Information:");
                    println!("  Version: {}", appstate.version);
                    println!("  Project path: {}", appstate.startpath);
                    println!("  Database file path: {}", appstate.Pathbindb);
                    println!("  Total courses: {}", appstate.db.len());

                    println!("\nCourse Details:");
                    for (idx, course) in appstate.db.iter().enumerate() {
                        println!("\n  Course {}: {}", idx + 1, course.courseName);
                        println!("    Total exercises: {}", course.questions.len());

                        if course.questions.len() > 0 {
                            println!("    Sample exercises:");
                            for (i, ex) in course.questions.iter().take(3).enumerate() {
                                println!("      {}. {} ({})", i + 1, ex.name, ex.language);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("✗ Failed to read database: {}", e);
                    println!("Note: Run test_initDB first to create the database");
                }
            }
        } else {
            println!("✗ Could not find praxis directory");
            println!("Make sure you're running this from within the praxis project");
        }
    }
}
