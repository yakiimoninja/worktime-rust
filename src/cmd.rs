use std::io::{stdin, stdout, Write};
use crate::sql::{inserttable, viewtable};

pub fn state1(){
    print!("Add date of work:");
    //let date_of_work
}

pub fn state2(){
    println!("This is the 2\n");
}

pub fn state3(){
    loop{

        println!("\n\t1. View all.\n\t2. Back.\n");

        let mut state: String = String::new();
        read(&mut state);

        //String to U8 and Invalid type error check
        let state: u8 = match state.trim().parse(){
            Ok(num) => num,
            Err(_) => {println!("Invalid input!\n"); continue;},
        };

        match state {
            1 => {println!("\nViewing table contents:\n"); viewtable(1).unwrap();},
            2 => {break;}
            _ => {println!("Invalid input!\n")}
        }
    }
}

pub fn read(input: &mut String){
    //Input from console function to string var
    print!("> ");
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read");
}
