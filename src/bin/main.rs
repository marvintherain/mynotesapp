extern crate mynotesapp;
extern crate diesel;

use self::mynotesapp::*;
use self::models::*;
use self::diesel::prelude::*;

use dialoguer::{theme::ColorfulTheme, Select, Editor};

fn main() {
    let connection = establish_connection();

    // let title = String::from("testtitle2");
    // let body = Some("this is a test note2");

    // let inserted = new_note(&connection, &title[..], body);

    // println!("{:?}", inserted);
//-------------------------------------------------
    let entry_selections = &[
        "create new note",
        "edit note",
        "delete note",
    ];
    let entry_selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(entry_selections)
            .interact()
            .unwrap();

    match entry_selection {
        0 => {
            println!("todo");
        },
        1 => {
                use schema::notes;
    
                let my_notes = notes::table
                .load::<Note>(&connection)
                .expect("Error loading notes");

                let mut ui_selections: Vec<String> = vec![];
                let my_copy = my_notes.clone();
                for entry in my_notes {
                    let entry_creation = entry.creation_date.format("%Y-%m-%d %H:%M:%S").to_string();
                    let new_selection = String::from(entry.title + " " + &entry_creation);
                    ui_selections.push(new_selection);
                }
                
                let edit_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("select the note you want to edit\ntitle              creation date")
                    .default(0)
                    .items(&ui_selections[..])
                    .interact()
                    .unwrap();

                let original_post = &my_copy[edit_selection].body.as_ref().unwrap();

                if let Some(rv) = Editor::new().edit(original_post).unwrap() {
                    let modded_note = mod_note(&connection, my_copy[edit_selection].id, None, Some(&rv));
                    println!("Note {} changed!", modded_note.title);
                } else {
                    println!("Abort");
                }

        },
        2 => {
            println!("todo");
        },
        _ => {
            println!("Error");
        }
    }

  

    // let deleted = delete_note(&connection, 2);

    // println!("{:?}", deleted);


}