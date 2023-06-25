// @generated automatically by Diesel CLI.

diesel::table! {
    assists (id) {
        id -> Int4,
        students_id -> Int4,
        presence -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 64]
        category -> Varchar,
        #[max_length = 150]
        section -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    payments (id) {
        id -> Int4,
        student_id -> Int4,
        amount -> Float4,
        type_payment_id -> Int4,
        service_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    people (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 150]
        first_name -> Varchar,
        #[max_length = 150]
        surname -> Nullable<Varchar>,
        user_id -> Int4,
        parent_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        person_id -> Int4,
        category_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    type_payments (id) {
        id -> Int4,
        #[max_length = 80]
        type_payment -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        active -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::joinable!(assists -> students (students_id));
diesel::joinable!(payments -> students (student_id));
diesel::joinable!(payments -> type_payments (type_payment_id));
diesel::joinable!(students -> categories (category_id));
diesel::joinable!(students -> people (person_id));
diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    assists,
    categories,
    payments,
    people,
    roles,
    students,
    type_payments,
    users,
    users_roles,
);
