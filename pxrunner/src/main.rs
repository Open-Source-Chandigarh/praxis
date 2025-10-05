use pxrunner::{Exercise, exercises};

fn main() {
    println!("Welcome to PX Runner!");
    let allquestion = exercises().unwrap();
    println!("{:?}",allquestion);
}
