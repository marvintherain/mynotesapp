use super::schema::{notes};

#[derive(Queryable, Identifiable, Debug, Clone)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub creation_date: chrono::NaiveDateTime,
    pub last_edit: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="notes"]
pub struct NewNote<'a> {
    pub title: &'a str,
    pub body: Option<&'a str>,
    pub creation_date: chrono::NaiveDateTime,
    pub last_edit: chrono::NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name="notes"]
pub struct ChangeNote<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub last_edit: chrono::NaiveDateTime,
}