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

use self::models::{Note, NewNote};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
