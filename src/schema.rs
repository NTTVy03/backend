// @generated automatically by Diesel CLI.

diesel::table! {
    todo_items (id) {
        id -> Uuid,
        title -> Text,
        completed -> Bool,
        create_at -> Timestamptz,
    }
}
