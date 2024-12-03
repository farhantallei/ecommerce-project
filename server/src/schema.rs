// @generated automatically by Diesel CLI.

diesel::table! {
    download_verifications (id) {
        id -> Text,
        product_id -> Text,
        expires_at -> Nullable<Int8>,
        created_at -> Int8,
    }
}

diesel::table! {
    orders (id) {
        id -> Text,
        user_id -> Text,
        product_id -> Text,
        price_in_cents -> Nullable<Int4>,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
    }
}

diesel::table! {
    products (id) {
        id -> Text,
        name -> Nullable<Text>,
        price_in_cents -> Nullable<Int4>,
        file_path -> Nullable<Text>,
        image_path -> Nullable<Text>,
        description -> Nullable<Text>,
        is_available_for_purchase -> Nullable<Bool>,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Nullable<Text>,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
    }
}

diesel::joinable!(download_verifications -> products (product_id));
diesel::joinable!(orders -> products (product_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    download_verifications,
    orders,
    products,
    users,
);
