// @generated automatically by Diesel CLI.

diesel::table! {
    album (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Nullable<Text>,
        picture_id -> Nullable<Uuid>,
        is_favorite -> Bool,
        is_hidden -> Bool,
        is_print_album -> Bool,
        is_printed -> Bool,
        image_per_page -> Nullable<Int4>,
        album_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    album_image (id) {
        id -> Uuid,
        album_id -> Uuid,
        image_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    album_rating_score (id) {
        id -> Uuid,
        album_id -> Uuid,
        rating_score_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    album_tag (id) {
        id -> Uuid,
        album_id -> Uuid,
        tag_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    image (id) {
        id -> Uuid,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        path -> Text,
        is_uploaded -> Bool,
        is_printable -> Bool,
        is_printed -> Bool,
        is_favorite -> Bool,
        is_hidden -> Bool,
        image_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    image_rating_score (id) {
        id -> Uuid,
        image_id -> Uuid,
        rating_score_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    image_tag (id) {
        id -> Uuid,
        image_id -> Uuid,
        tag_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    kitty (id) {
        id -> Uuid,
        name -> Varchar,
        names -> Nullable<Text>,
        picture_id -> Nullable<Uuid>,
        album_id -> Nullable<Uuid>,
        description -> Nullable<Text>,
        age -> Nullable<Int4>,
        origin -> Nullable<Varchar>,
        is_favorite -> Bool,
        fc -> Int4,
        wsic -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    kitty_album (id) {
        id -> Uuid,
        kitty_id -> Uuid,
        album_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    kitty_image (id) {
        id -> Uuid,
        image_id -> Uuid,
        kitty_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    kitty_rating_score (id) {
        id -> Uuid,
        kitty_id -> Uuid,
        rating_score_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    kitty_tag (id) {
        id -> Uuid,
        kitty_id -> Uuid,
        tag_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    link (id) {
        id -> Uuid,
        title -> Varchar,
        url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rating (id) {
        id -> Uuid,
        name -> Varchar,
        scale -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rating_score (id) {
        id -> Uuid,
        rating_id -> Uuid,
        score -> Int4,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tag (id) {
        id -> Uuid,
        name -> Varchar,
        tag_category_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tag_category (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        hash -> Varchar,
        fp -> Int4,
        wsic -> Int4,
        is_admin -> Bool,
        subscription -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(album -> image (picture_id));
diesel::joinable!(album_image -> album (album_id));
diesel::joinable!(album_image -> image (image_id));
diesel::joinable!(album_rating_score -> album (album_id));
diesel::joinable!(album_rating_score -> rating_score (rating_score_id));
diesel::joinable!(album_tag -> album (album_id));
diesel::joinable!(album_tag -> tag (tag_id));
diesel::joinable!(image_rating_score -> image (image_id));
diesel::joinable!(image_rating_score -> rating_score (rating_score_id));
diesel::joinable!(image_tag -> image (image_id));
diesel::joinable!(image_tag -> tag (tag_id));
diesel::joinable!(kitty -> album (album_id));
diesel::joinable!(kitty -> image (picture_id));
diesel::joinable!(kitty_album -> album (album_id));
diesel::joinable!(kitty_album -> kitty (kitty_id));
diesel::joinable!(kitty_image -> image (image_id));
diesel::joinable!(kitty_image -> kitty (kitty_id));
diesel::joinable!(kitty_rating_score -> kitty (kitty_id));
diesel::joinable!(kitty_rating_score -> rating_score (rating_score_id));
diesel::joinable!(kitty_tag -> kitty (kitty_id));
diesel::joinable!(kitty_tag -> tag (tag_id));
diesel::joinable!(rating_score -> rating (rating_id));
diesel::joinable!(tag -> tag_category (tag_category_id));

diesel::allow_tables_to_appear_in_same_query!(
    album,
    album_image,
    album_rating_score,
    album_tag,
    image,
    image_rating_score,
    image_tag,
    kitty,
    kitty_album,
    kitty_image,
    kitty_rating_score,
    kitty_tag,
    link,
    rating,
    rating_score,
    tag,
    tag_category,
    users,
);
