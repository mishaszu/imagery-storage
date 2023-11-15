// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Uuid,
        #[max_length = 255]
        fullname -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        kind -> Varchar,
        is_admin -> Bool,
        public_lvl -> Int4,
        is_banned -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        is_wall -> Bool,
        picture -> Nullable<Uuid>,
        public_lvl -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    album_post (id) {
        id -> Uuid,
        album_id -> Uuid,
        post_id -> Uuid,
    }
}

diesel::table! {
    comment (id) {
        id -> Uuid,
        user_id -> Uuid,
        post_id -> Uuid,
        body -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    fav (id) {
        id -> Uuid,
        user_id -> Uuid,
        post_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    image (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Nullable<Text>,
        kind -> Text,
        path -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    post (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        user_id -> Uuid,
        add_to_feed -> Bool,
        disable_comments -> Bool,
        public_lvl -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    post_image (id) {
        id -> Uuid,
        post_id -> Uuid,
        image_id -> Uuid,
    }
}

diesel::table! {
    post_tag (id) {
        id -> Uuid,
        post_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    referral (id) {
        id -> Uuid,
        referrer_id -> Uuid,
        user_id -> Uuid,
        expires_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    sys_config (id) {
        id -> Uuid,
        allow_registration -> Bool,
        single_user_feed -> Nullable<Uuid>,
    }
}

diesel::table! {
    tag (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_picture (id) {
        id -> Uuid,
        user_id -> Uuid,
        image_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        account_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(album -> account (user_id));
diesel::joinable!(album -> image (picture));
diesel::joinable!(album_post -> album (album_id));
diesel::joinable!(album_post -> post (post_id));
diesel::joinable!(comment -> post (post_id));
diesel::joinable!(comment -> users (user_id));
diesel::joinable!(fav -> post (post_id));
diesel::joinable!(fav -> users (user_id));
diesel::joinable!(image -> users (user_id));
diesel::joinable!(post -> users (user_id));
diesel::joinable!(post_image -> image (image_id));
diesel::joinable!(post_image -> post (post_id));
diesel::joinable!(post_tag -> post (post_id));
diesel::joinable!(post_tag -> tag (tag_id));
diesel::joinable!(user_picture -> image (image_id));
diesel::joinable!(user_picture -> users (user_id));
diesel::joinable!(users -> account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    album,
    album_post,
    comment,
    fav,
    image,
    post,
    post_image,
    post_tag,
    referral,
    sys_config,
    tag,
    user_picture,
    users,
);
