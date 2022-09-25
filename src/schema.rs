// @generated automatically by Diesel CLI.

diesel::table! {
    uploaded_files (path) {
        path -> Text,
        hash -> Text,
    }
}
