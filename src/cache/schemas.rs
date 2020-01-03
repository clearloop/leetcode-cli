//! Leetcode data schemas
table! {
    problems(id) {
        category -> Text,
        fid -> Integer,
        id -> Integer,
        level -> Integer,
        locked -> Bool,
        name -> Text,
        percent -> Float,
        slug -> Text,
        starred -> Bool,
        status -> Text,
    }
}
