table! {
    images (id) {
        id -> Int4,
        site_id -> Int4,
        ref_name -> Varchar,
        ref_path -> Nullable<Varchar>,
    }
}

table! {
    texts (id) {
        id -> Int4,
        site_id -> Int4,
        ref_name -> Varchar,
        content -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    images,
    texts,
);
