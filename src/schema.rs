// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Uuid,
        referral_id -> Nullable<Uuid>,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        kind -> Varchar,
        is_admin -> Bool,
        is_public -> Bool,
        is_active -> Bool,
        is_banned -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    album (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        picture -> Nullable<Uuid>,
        is_public -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    album_image (id) {
        id -> Uuid,
        album_id -> Uuid,
        image_id -> Uuid,
    }
}

diesel::table! {
    comment (id) {
        id -> Uuid,
        user_id -> Uuid,
        image_id -> Uuid,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fav (id) {
        id -> Uuid,
        user_id -> Uuid,
        image_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    follow (id) {
        id -> Uuid,
        follower_id -> Uuid,
        followee_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    image (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        path -> Uuid,
        is_public -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    image_tag (id) {
        id -> Uuid,
        image_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    sys_config (id) {
        id -> Uuid,
        allow_registration -> Bool,
    }
}

diesel::table! {
    tag (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    theme (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        color -> Varchar,
        picture -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        nick -> Varchar,
        #[max_length = 255]
        hash -> Varchar,
        #[max_length = 255]
        access_key -> Nullable<Varchar>,
        #[max_length = 255]
        picture -> Nullable<Varchar>,
        is_public -> Bool,
        account_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(album -> account (user_id));
diesel::joinable!(album -> image (picture));
diesel::joinable!(album_image -> album (album_id));
diesel::joinable!(album_image -> image (image_id));
diesel::joinable!(comment -> image (image_id));
diesel::joinable!(comment -> users (user_id));
diesel::joinable!(fav -> image (image_id));
diesel::joinable!(fav -> users (user_id));
diesel::joinable!(image -> users (user_id));
diesel::joinable!(image_tag -> image (image_id));
diesel::joinable!(image_tag -> tag (tag_id));
diesel::joinable!(theme -> image (picture));
diesel::joinable!(users -> account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    album,
    album_image,
    comment,
    fav,
    follow,
    image,
    image_tag,
    sys_config,
    tag,
    theme,
    users,
);
