pub mod indexer{
use std::{collections::HashMap, env, fs::{self, DirEntry}, io::{self, Error, ErrorKind}};

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
    pub passed : HashMap<String,bool>
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
fn fetch_remote_db(lang: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://raw.githubusercontent.com/Open-Source-Chandigarh/praxis/main/practicals/{}/db.txt",
        lang
    );
    let content = reqwest::blocking::get(url)?.text()?;
    Ok(content)
}
fn fetch_local_db() -> Result<String, Box<dyn std::error::Error>> {
    
    let path = format!("./db.txt");
    let content = fs::read_to_string(path).expect("No db detected! PLease px init in new directory");
    Ok(content)
}

pub fn syncdbs()-> bool{
    let remotedb = fetch_remote_db("rust").unwrap();
    let localdb = fetch_local_db().unwrap();
    let mut remoteresult = Vec::<&str>::new();
    let mut localresult = Vec::<&str>::new();
    for remotelines in remotedb.lines(){
        let filenameresult = remotelines.split(' ').next().unwrap();
        remoteresult.push(filenameresult);
    }
     for locallines in localdb.lines(){
        let filenameresult = locallines.split(' ').next().unwrap();
        localresult.push(filenameresult);
    }
    return localresult==remoteresult;

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
                        let modulename =dir.file_name().to_string_lossy().into_owned();
                        let key = format!("{}/{}", modulename, questionname);
                        let pathname = format!("./{}/{}", modulename, questionname);
                         let ispassed = allquestions.passed.get(&key).copied().unwrap_or(false);
                        allquestions.add(questionname.clone(), ispassed, getlanguagename().unwrap().clone(), modulename.clone(), pathname );            
                    }
                }
            }
        }
    }
                   
}
pub fn exercises() -> Result<Vec<Exercise>, std::io::Error> {
    let mut allquestions = Exercises::default();
    if !syncdbs(){
        return Err(Error::new(ErrorKind::Other, "Db's Altered! Please init in a new directory"));
    }
    let result = fetch_local_db().expect("Error reading local DB");
    for line in result.lines() {
        let mut content = line.split(' ');
        let key = content.next().unwrap().to_string();
        let value = content.next().unwrap().to_string().parse::<bool>().unwrap();
        allquestions.passed.insert(key, value);
        
    }

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