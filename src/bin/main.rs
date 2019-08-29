extern crate mynotesapp;
extern crate diesel;

use self::mynotesapp::*;
use self::models::*;
use self::diesel::prelude::*;

use std::io;

use dialoguer::{theme::ColorfulTheme, Select, Editor, Checkboxes};

fn main() {
    let connection = establish_connection();

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
            //create note
            println!("enter title of new note");
            let mut title = String::new();
            io::stdin().read_line(&mut title).expect("line could not be read");
            //remove the \n at the end of title
            let title = &title[..title.len()-1];

            if let Some(rv) = Editor::new()
                .edit("Enter new note body here")
                .unwrap() {
                
                let my_new_note = new_note(&connection, &title, Some(&rv));
                    println!("note {} created!", my_new_note.title);
                } else {
                    println!("abort");
                }

        },
        1 => {
                //edit note
                let my_notes = read_notes(&connection);
                let my_copy = my_notes.clone();

                let ui_selections = reformat_notes_for_ui(my_notes);
                
                let edit_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("select the note you want to edit\ntitle              creation date")
                    .default(0)
                    .items(&ui_selections[..])
                    .interact()
                    .unwrap();

                let original_post = &my_copy[edit_selection]
                    .body
                    .as_ref()
                    .unwrap();

                if let Some(rv) = Editor::new().edit(original_post).unwrap() {
                    let modded_note = mod_note(&connection, my_copy[edit_selection].id, None, Some(&rv));
                    println!("Note {} changed!", modded_note.title);
                } else {
                    println!("Abort");
                }

        },
        2 => {
            //delete note
            let my_notes = read_notes(&connection);
            let my_copy = my_notes.clone();

            let ui_selections = reformat_notes_for_ui(my_notes);
            let ui_selections: Vec<&str> = ui_selections.iter().map(|x| &**x).collect();

            let notes_to_delete = Checkboxes::with_theme(&ColorfulTheme::default())
                .with_prompt("choose the notes to delete")
                .items(&ui_selections[..])
                .interact()
                .unwrap();
            
            for i in notes_to_delete {
                let deleted = delete_note(&connection, my_copy[i].id);
                println!("deleted note {}", deleted.title);
            }
            
            

        },
        _ => {
            println!("Error");
        }
    }


}