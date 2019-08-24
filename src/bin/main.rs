extern crate mynotesapp;
extern crate diesel;

use self::mynotesapp::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    let connection = establish_connection();

    // let title = String::from("testtitle1");
    // let body = Some("this is a test note1");

    // let inserted = new_note(&connection, &title[..], body);

    // println!("{:?}", inserted);

    use schema::notes;

    let note_id: i32 = 1;

    let filtered_note = notes::table.filter(notes::id.eq(note_id));

    let deleted: Note = diesel::delete(filtered_note)
        .get_result(&connection)
        .expect("error deleting note");

    println!("{:?}", deleted);
}