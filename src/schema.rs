// @generated automatically by Diesel CLI.

diesel::table! {
    assists (id) {
        id -> Int4,
        students_id -> Int4,
        presence -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(assists -> students (students_id));

diesel::allow_tables_to_appear_in_same_query!(
    assists,
    students,
);
