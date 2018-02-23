table! {
    queues (id) {
        id -> Varchar,
        title -> Varchar,
        frozen -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        user_id -> Varchar,
        queue_id -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(users -> queues (queue_id));

allow_tables_to_appear_in_same_query!(queues, users,);
