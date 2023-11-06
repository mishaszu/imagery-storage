// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Uuid,
        referral_id -> Nullable<Uuid>,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        kind -> Varchar,
        followee_id -> Nullable<Uuid>,
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
    follow (id) {
        id -> Uuid,
        follower_id -> Uuid,
        followee_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    image (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Nullable<Text>,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        user_id -> Uuid,
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
        #[max_length = 255]
        access_key -> Nullable<Varchar>,
        #[max_length = 255]
        picture -> Nullable<Varchar>,
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
diesel::joinable!(theme -> account (user_id));
diesel::joinable!(theme -> image (picture));
diesel::joinable!(users -> account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    album,
    album_post,
    comment,
    fav,
    follow,
    image,
    post,
    post_image,
    post_tag,
    sys_config,
    tag,
    theme,
    users,
);
