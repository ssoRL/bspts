table! {
    tasks (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        bspts -> Nullable<Int4>,
        is_done -> Bool,
        next_reset -> Nullable<Date>,
        frequency -> Nullable<Interval>,
    }
}
