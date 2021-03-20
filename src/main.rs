#![allow(non_snake_case)]
pub mod sql;
use sql::{inserttable, tablecreation};
pub mod cmd;
use cmd::*;

fn main() {

    println!("\nWelcome to WorkTime!\n");

    //This is the main application loop
    loop{
        
        //Welcome message
        println!("\t1. Add a new entry.\n\t2. Edit existing entry.\n\t3. View entries.\n\t4. Exit.\n");
        
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
            2 => {println!("This is the 2\n")},
            3 => {state3();},
            4 => {println!("Exiting application!\n"); break},
            _ => println!("Invalid input!\n"),
        }
    }
}

