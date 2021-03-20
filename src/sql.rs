#![allow(non_snake_case)]
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Workentry {
    id: u32,
    hours_worked: String,
    date_of_work: String,
    date_of_edit: String,
}

pub fn tablecreation() -> Result<()> {

    //Establishing connection to database
    let conn = Connection::open("test.db")?;
    
    //Creating table if it doesnt exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS workcalendar(
                  id              INTEGER PRIMARY KEY AUTOINCREMENT,
                  hours_worked    TEXT NOT NULL,
                  date_of_work    TEXT NOT NULL,   
                  date_of_edit    TEXT
                  )",
        params![],
    )?;
    
    //Calling function that insterts to table
    //inserttable(&conn).expect("Insert didnt work");

    //Calling function that prints table contents
    //viewtable(&conn).expect("View didnt work");

    Ok(())

}


//Function to view table contents
pub fn viewtable(_x: u8) -> Result<()>{

    let conn = Connection::open("test.db")?;
    //Sql query for printing table contents
    let mut stmt = conn.prepare("SELECT id, hours_worked, date_of_work, date_of_edit FROM workcalendar")?;
    let table_iter = stmt.query_map(params![], |row| {
        let id: u32 = row.get(0)?;
        let hours_worked: String = row.get(1)?;
        let date_of_work: String = row.get(2)?;
        let date_of_edit: String = row.get(3)?;
        
        Ok((id, hours_worked, date_of_work, date_of_edit))
    })?;

    //Printing table contents to console
    for tc in table_iter {
        let tc_string: String = format!("{:?}", tc.unwrap());
        string_fmt(tc_string);
    }
    Ok(())

}


//Function to insest to table
pub fn inserttable() -> Result<()> {

    let conn = Connection::open("test.db")?;
    //Values to be inserted
    let me = Workentry {
        id: 0,
        hours_worked: "3".to_string(),
        date_of_work: "kapote".to_string(),
        date_of_edit: "ne re mpor".to_string(),
    };
    
    //Sql query of inserting values
    conn.execute(
        "INSERT INTO workcalendar (hours_worked, date_of_work, date_of_edit) VALUES (?1, ?2, ?3)",
        params![me.hours_worked, me.date_of_work, me.date_of_edit],
    )?;

    Ok(())

}


//Function to reformat contents of table output
fn string_fmt(mut x: String){

    x = x.replace("(", "Id: ");
    x = x.replacen(", \"", "    Hours: ", 1);
    x = x.replacen("\", \"", "    Date: ", 1);
    x = x.replace("\", \"", "    Edited on: ");
    x = x.replace("\")", "");
    println!("{}", x);
}

//fn edittable(){
//
//}
