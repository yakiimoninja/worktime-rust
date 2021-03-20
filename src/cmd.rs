use std::io::{stdin, stdout, Write};
use crate::sql::{inserttable, viewtable};
extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;
use rusqlite::ToSql;
use std::time::SystemTime;

pub fn state1(){

    loop{

        //Navigation
        println!("\t1. Add entry.\n\t2. Delete entry.\n\t3. View entries\n\t4. Back.\n");

        //Navigation
        let mut state: String = String::new();
        read(&mut state);
        let mut state: u8 = match state.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Invalid input!"); continue;}
        };

        //More navigation
        match state {
            1 => insertprep(),
            2 => print!(""),
            3 => {println!("\nViewing table contents:\n"); viewtable(1).unwrap();},
            4 => {break;}
            _ => print!(""),
        }

    }
}

pub fn state2(){
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

//Prep for sql query
fn insertprep(){
    //Data for table inserion
    print!("\nAdd date of work:");

    let mut date: String = String::new();
    read(&mut date);

    println!("");

    let system_date = SystemTime::now();
    let datetime: DateTime<Local> = system_date.into();
    let datetime: String = format!("{}", datetime.format("%d/%m/%Y"));
    
    //Loop for int data type check
    loop{
        print!("Add hours: ");
        let mut hours: String = String::new();
        read(&mut hours);
        println!("");

        let hours: u32 = match hours.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Invalid input!\n"); continue;},
        };
        
    //Function call for data inserion
    inserttable(hours, date, datetime).expect("Insertion error");

    println!("Data submitted successfully!\n\n");
        
    break;
    }
}
