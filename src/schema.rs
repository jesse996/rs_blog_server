table! {
    users (id) {
        id -> Varchar,
        uname -> Varchar,
        password -> Varchar,
        join_at -> Timestamp,
        avatar -> Varchar,
        email -> Varchar,
        intro -> Varchar,
        location -> Varchar,
        nickname -> Varchar,
        permission -> Int2,
        link -> Varchar,
        auth_from -> Varchar,
        email_confirmed -> Bool,
    }
}
