table! {
    rewards (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        description -> Text,
        bspts -> Int4,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        bspts -> Int4,
        is_done -> Bool,
        next_reset -> Date,
        every -> Int4,
        time_unit -> Text,
        by_when -> Int4,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        uname -> Text,
        password -> Bytea,
        salt -> Bytea,
        bspts -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    rewards,
    sessions,
    tasks,
    users,
);
