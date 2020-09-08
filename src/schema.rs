table! {
    about (id) {
        id -> Int4,
        site_id -> Int4,
        ref_name -> Varchar,
        content -> Nullable<Varchar>,
    }
}

table! {
    blog (id) {
        id -> Int4,
        site_id -> Int4,
        post -> Nullable<Varchar>,
        img_path -> Nullable<Varchar>,
        posted_at -> Timestamp,
    }
}

table! {
    contact (id) {
        id -> Int4,
        site_id -> Int4,
        name -> Varchar,
        email -> Varchar,
        message -> Varchar,
    }
}

table! {
    images (id) {
        id -> Int4,
        site_id -> Int4,
        ref_name -> Varchar,
        ref_path -> Nullable<Varchar>,
    }
}

table! {
    service (id) {
        id -> Int4,
        site_id -> Int4,
        content -> Nullable<Varchar>,
    }
}

table! {
    sites (id) {
        id -> Int4,
        ref_name -> Varchar,
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

table! {
    users (id) {
        id -> Int4,
        site_id -> Int4,
        ref_name -> Varchar,
        email -> Varchar,
        pass -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    about,
    blog,
    contact,
    images,
    service,
    sites,
    texts,
    users,
);
