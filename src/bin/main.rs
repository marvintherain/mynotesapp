extern crate mynotesapp;
extern crate diesel;

use self::mynotesapp::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    let connection = establish_connection();

    let title = String::from("testtitle1");
    let body = Some("this is a test note1");

    let inserted = new_note(&connection, &title[..], body);

    println!("{:?}", inserted);
}