pub mod indexer{
use std::{collections::HashMap, env, fs::{self, DirEntry}, io};
#[derive(Debug,Default)]
#[warn(dead_code)]
pub struct Exercise{
    name:String,
    passed:bool,
    language:String,
    parentmodule:String,
    path:String
}
#[derive(Debug,Default)]
pub struct Exercises{
    pub questions:Vec<Exercise>,
    pub passed:HashMap<String,bool>
}
impl Exercises{
    fn add(&mut self,name:String,passed:bool,language:String,parentmodule:String,path:String){
        let question = Exercise{
            name:name,
            passed:passed,
            language:language,
            parentmodule:parentmodule,
            path:path
        };
        self.questions.push(question);

    }
}
fn getlanguagename() -> Result<String,io::Error>{
    let languagename = match env::current_dir() {
    Ok(dirname) => match dirname.file_name() {
        Some(languagename) => Ok(languagename.to_string_lossy().into_owned()),
        None => return Err(io::Error::new(io::ErrorKind::Other, "No directory name found")),
    },
    Err(e) => return Err(e)
  };
  languagename
}
fn makeexercise(dir:&DirEntry,allquestions:&mut Exercises){
     if let Ok(subentries) =fs::read_dir(dir.path()){
            for subentry in subentries{
                if let Ok(subdirs) = subentry{
                    if let Ok(questions) = subdirs.file_type(){
                        if !questions.is_file(){
                            let questionname =subdirs.file_name().to_string_lossy().into_owned();
                            println!("{}",questionname);
                                let modulename =dir.file_name().to_string_lossy().into_owned();
                                let exercise = allquestions.add(questionname.clone(), false, getlanguagename().unwrap().clone(), modulename.clone(),  format!("./{}/{}", modulename, questionname));            
                        }
                    }
                }
            }
        }
                   
}
pub fn checkpassed(){
    
}
pub fn exercises() -> Result<Vec<Exercise>, std::io::Error> {
    let mut allquestions = Exercises::default();
    if let Ok(entries)  = fs::read_dir("."){
        for entry in entries{
            if let Ok(dir) = entry{
                if let Ok(file) = dir.file_type(){
                   if !file.is_file() && dir.file_name()!=".git"{
                    makeexercise(&dir,&mut allquestions);
                   }
                }
            }
        }
    }

    Ok(allquestions.questions)
}
}