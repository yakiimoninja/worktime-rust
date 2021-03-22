#![allow(non_snake_case)]
pub mod sql;
use sql::tablecreation;
pub mod cmd;
use cmd::*;

fn main() {

    println!("\nWelcome to WorkTime!");

    //This is the main application loop
    loop{
        
        //Welcome message
        println!("\n\t1. Edit entries.\n\t2. View entries.\n\t0. Exit.\n");
        
        //Initializing SQL database
        tablecreation().unwrap();

        //Console input to string
        let mut state: String = String::new();
        read(&mut state);

        //String to U8 and Invalid type error check
        let state: u8 = match state.trim().parse(){
            Ok(num) => num,
            Err(_) => {println!("Invalid input!\n"); continue},
        };

        //Logic for navigation
        match state {
            1 => {state1();},
            2 => {state2();},
            0 => {println!("Exiting application!\n"); break},
            _ => println!("Invalid input!\n"),
        }
    }
}

