#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use chrono::prelude::{Utc};

use std::env;

use self::models::{Note, NewNote, ChangeNote};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn read_notes(conn: &PgConnection) -> Vec<Note> {
        use schema::notes;

        notes::table
                .load::<Note>(conn)
                .expect("Error loading notes")
}

pub fn reformat_notes_for_ui(notes: Vec<Note>) -> Vec<String> {
        let my_notes = notes;

        let mut ui_selections: Vec<String> = vec![];
        for entry in my_notes {
                let entry_creation = entry
                        .creation_date
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string();
                let new_selection = String::from(entry.title + " " + &entry_creation);
                ui_selections.push(new_selection);
        };

        return ui_selections        
}

pub fn new_note<'a>(conn: &PgConnection, title: &'a str, body: Option<&'a str>) -> Note {
    use schema::notes;

    let now = Utc::now().naive_utc();

    let new_note = NewNote {
        title: title,
        body: body,
        creation_date: now,
        last_edit: now,
    };

    diesel::insert_into(notes::table)
        .values(&new_note)
        .get_result(conn)
        .expect("Error saving new note")
}

pub fn mod_note<'a>(conn: &PgConnection, note_id: i32, title: Option<&'a str>, body: Option<&'a str>) -> Note {
    use schema::notes;    
    
    let now = Utc::now().naive_utc();
    
    let modded_note = ChangeNote {
        title: title,
        body: body,
        last_edit: now,
    };

    let target = notes::table.filter(notes::id.eq(note_id));

    diesel::update(target)
        .set(&modded_note)
        .get_result(conn)
        .expect("Error changing note")
}

pub fn delete_note<'a>(conn: &PgConnection, note_id: i32) -> Note {
    use schema::notes;

    let filtered_note = notes::table.filter(notes::id.eq(note_id));

    diesel::delete(filtered_note)
        .get_result(conn)
        .expect("Error deleting note")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
