use pxrunner::gitcommands::{app, cli, implementations, traits};
use pxrunner::indexerv2::modernindx::{checkWorkDir, trywalk};
use pxrunner::{Exercise, exercises};
//
// fn main() {
//     app::run();
// }
//
fn main() {
    // let dircheck = checkWorkDir().unwrap();
    // println!("{:?}", dircheck);
    let dirs = trywalk();
    for dir in dirs {
        println!("{:?}", dir);
    }
}
