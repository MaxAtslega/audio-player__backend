// @generated automatically by Diesel CLI.

diesel::table! {
    audio_audios (id) {
        id -> Int4,
        name -> Varchar,
        file -> Varchar,
        category -> Varchar,
        organisation_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    organisation_organisations (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    organisation_users (organisation_id, user_id) {
        organisation_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    user_users (id) {
        id -> Int4,
        key -> Varchar,
        role -> Varchar,
    }
}

diesel::joinable!(organisation_users -> user_users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    audio_audios,
    organisation_organisations,
    organisation_users,
    user_users,
);
