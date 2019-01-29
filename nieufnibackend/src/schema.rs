table! {
    articles (db_id) {
        id -> Varchar,
        db_id -> Int4,
        title -> Varchar,
        date -> Varchar,
        authors_id -> Int4,
        markdown_text -> Text,
        rendered_text -> Text,
        published -> Bool,
    }
}

table! {
    authors (id) {
        id -> Int4,
        name -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(articles -> authors (authors_id));

allow_tables_to_appear_in_same_query!(
    articles,
    authors,
);
