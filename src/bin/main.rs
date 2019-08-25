extern crate mynotesapp;
extern crate diesel;

use self::mynotesapp::*;
use self::models::*;
use self::diesel::prelude::*;

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let connection = establish_connection();

    // let title = String::from("testtitle2");
    // let body = Some("this is a test note2");

    // let inserted = new_note(&connection, &title[..], body);

    // println!("{:?}", inserted);

//-------------------------------------------------

    use schema::notes;

    // let my_notes: Vec<(i32, String, chrono::NaiveDateTime, chrono::NaiveDateTime)> = notes::table
    //     .select((notes::id,
    //              notes::title, 
    //              notes::creation_date, 
    //              notes::last_edit))
    //     .load(&connection)
    //     .expect("Error loading notes");
    
    let my_notes = notes::table
     .load::<Note>(&connection)
     .expect("Error loading notes");

    println!("{:?}\n", my_notes[0].title);
    let mut ui_selections: Vec<String> = vec![];
    // println!("title    creation date    last edit\n");
    let my_copy = my_notes.clone();
    for entry in my_notes {
        let entry_creation = entry.creation_date.format("%Y-%m-%d %H:%M:%S").to_string();
        let new_selection = String::from(entry.title + " " + &entry_creation);
        ui_selections.push(new_selection);
    }
    // println!("{:?}", ui_selections);

//-------------------------------------------------
    

    // let ui_selections = &my_notes;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("select the note you want to edit\ntitle              creation date")
        .default(0)
        .items(&ui_selections[..])
        .interact()
        .unwrap();
    println!("Enjoy your {}!", ui_selections[selection]);
    println!("{:?}", my_copy[selection]);


    // let deleted = delete_note(&connection, 2);

    // println!("{:?}", deleted);


}