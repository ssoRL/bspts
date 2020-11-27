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
    }
}
