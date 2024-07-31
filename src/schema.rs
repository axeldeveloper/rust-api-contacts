// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Int4,
        name -> Varchar,
        cpf -> Varchar,
        age -> Nullable<Int4>,
        published -> Bool,
    }
}
