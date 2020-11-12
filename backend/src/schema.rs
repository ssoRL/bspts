table! {
    tasks (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        bspts -> Int4,
        is_done -> Bool,
        next_reset -> Date,
        frequency -> Interval,
    }
}
