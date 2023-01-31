table! {
    channels (id) {
        id -> Int8,
        remote -> Text,
        slug -> Text,
        d_title -> Text,
        d_link -> Text,
        d_description -> Text,
        d_categories -> Array<Text>,
        d_image -> Nullable<Text>,
        d_author -> Nullable<Text>,
        d_subtitle -> Nullable<Text>,
        c_image -> Nullable<Text>,
    }
}

table! {
    items (id) {
        id -> Int8,
        channel -> Int8,
        has_enclosure -> Bool,
        discovered -> Timestamptz,
        d_title -> Nullable<Text>,
        d_link -> Nullable<Text>,
        d_description -> Nullable<Text>,
        d_author -> Nullable<Text>,
        d_categories -> Array<Text>,
        d_encl_url -> Nullable<Text>,
        d_encl_length -> Nullable<Text>,
        d_encl_mime_type -> Nullable<Text>,
        d_guid -> Nullable<Text>,
        d_pub_date -> Nullable<Timestamptz>,
        d_content -> Nullable<Text>,
        d_image -> Nullable<Text>,
        d_duration -> Nullable<Int4>,
        c_encl_url -> Nullable<Text>,
        c_image -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    channels,
    items,
);
