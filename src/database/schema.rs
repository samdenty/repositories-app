table! {
    repo_tags (repo_id) {
        repo_id -> Integer,
        tag_name -> Text,
    }
}

table! {
    repos (id) {
        id -> Integer,
        user_name -> Text,
        name -> Text,
        description -> Nullable<Text>,
        private -> Bool,
        fork -> Bool,
    }
}

table! {
    tags (name) {
        name -> Text,
    }
}

table! {
    users (name) {
        name -> Text,
        description -> Nullable<Text>,
    }
}

joinable!(repo_tags -> repos (repo_id));
joinable!(repo_tags -> tags (tag_name));
joinable!(repos -> users (user_name));

allow_tables_to_appear_in_same_query!(
    repo_tags,
    repos,
    tags,
    users,
);
