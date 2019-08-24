table! {
    notes (id) {
        id -> Int4,
        title -> Varchar,
        body -> Nullable<Text>,
        creation_date -> Timestamp,
        last_edit -> Timestamp,
    }
}
