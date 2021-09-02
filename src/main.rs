#![allow(non_snake_case)]
pub mod sql;
use sql::tablecreation;
pub mod cmd;
use cmd::*;

fn main() {

    println!("\nWelcome to WorkTime!\n");

    //Initializing SQL database
    tablecreation().unwrap();

    //Calling the logic loop
    state1();
}

