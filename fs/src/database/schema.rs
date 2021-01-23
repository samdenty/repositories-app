table! {
    blobs (sha) {
        sha -> Text,
        size -> Integer,
        data -> Nullable<Binary>,
    }
}

table! {
    repo_branches (owner, repo, name) {
        owner -> Text,
        repo -> Text,
        name -> Text,
        tree_sha -> Text,
    }
}

table! {
    repo_tags (owner, repo, tag_name) {
        owner -> Text,
        repo -> Text,
        tag_name -> Text,
    }
}

table! {
    repos (owner, name) {
        owner -> Text,
        name -> Text,
        default_branch -> Text,
        description -> Nullable<Text>,
        homepage -> Nullable<Text>,
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
    trees (sha, path) {
        sha -> Text,
        path -> Text,
        mode -> Text,
        blob_sha -> Nullable<Text>,
    }
}

table! {
    users (name) {
        name -> Text,
        description -> Nullable<Text>,
    }
}

joinable!(repo_tags -> tags (tag_name));
joinable!(repos -> users (owner));
joinable!(trees -> blobs (blob_sha));

allow_tables_to_appear_in_same_query!(blobs, repo_branches, repo_tags, repos, tags, trees, users,);
