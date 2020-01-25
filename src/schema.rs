table! {
    hero (id) {
        id -> Int4,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
