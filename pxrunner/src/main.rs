use pxrunner::{Exercise, exercises};
use pxrunner::{app_logic, cli, github_implementations, github_traits};

fn main() {
    println!("Welcome to PX Runner!");
    let allquestion = exercises().unwrap();
    println!("{:?}", allquestion);
}
