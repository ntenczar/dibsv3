table! {
    queues (id) {
        id -> Varchar,
        title -> Varchar,
        frozen -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
        user_id -> Varchar,
        queue_id -> Varchar,
    }
}

joinable!(users -> queues (queue_id));

allow_tables_to_appear_in_same_query!(queues, users,);
