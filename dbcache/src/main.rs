use dbcache::{git_clone_exercises, serializer, update_dbs};
use std::fs;
use std::path::Path;


fn main(){
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    git_clone_exercises();
    let languages = update_dbs().unwrap();
    serializer(languages);
    std::env::set_current_dir(&original_dir).expect("Failed to return to original directory");
    let db_bin_path = Path::new("db.bin");
    if db_bin_path.exists() {
        fs::remove_file(db_bin_path).unwrap();
    }
    let src_path = Path::new("praxis/db.bin");
    if src_path.exists() {
        fs::rename(src_path, db_bin_path).unwrap();
        println!("Moved db.bin to dbcache root directory");
    } else {
        println!("Warning: praxis/db.bin not found");
    }

}