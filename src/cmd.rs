use std::io::{stdin, stdout, Write};
use crate::sql::{deleteentry, insertentry, viewtable};
extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;
use std::time::SystemTime;

pub fn state1(){

    loop{

        //Navigation
        println!("\t1. Add entry.\n\t2. Delete entry.\n\t3. View entries\n\t0. Exit.\n");

        //Console input to string
        let mut state: String = String::new();
        read(&mut state);
        //String to U8 and Invalid type error check
        let state: u8 = match state.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("\nInvalid input!\n"); continue;}
        };

        //More navigation
        match state {
            1 => insertprep(),
            2 => {
                println!("");
                viewtable(1).unwrap();
                print!("\nChoose entry to delete by Id:");
                del();
                },
            3 => {println!("\nViewing table contents:\n"); viewtable(1).unwrap(); println!("");},
            0 => {break;}
            _ => print!(""),
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
    insertentry(hours, date, datetime).expect("Insertion error");

    println!("Data submitted successfully!\n");
        
    break;
    }
}

fn del(){

    loop {

        let mut nav: String = String::new();
        read(&mut nav);
        let nav: u8 = match nav.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Invalid input!"); continue;}
        };

        deleteentry(nav).expect("");
        
        println!("");
        viewtable(1).unwrap();

        println!("\nEntry deleted successfully!\n");
        break;
    }
}
