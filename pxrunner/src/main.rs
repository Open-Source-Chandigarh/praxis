use pxrunner::{Exercise, exercises,checkpassed};

fn main() {
    println!("Welcome to PX Runner!");
    let allquestion = exercises().unwrap();
    println!("{:?}",allquestion);
}
