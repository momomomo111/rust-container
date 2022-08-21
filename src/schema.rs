table! {
    friend (owner_id, friend_name) {
        owner_id -> Integer,
        friend_name -> Integer,
    }
}

table! {
    gohobi (id) {
        id -> Integer,
        message -> Varchar,
        from_user_id -> Integer,
        task_id -> Integer,
    }
}

table! {
    small_task (id) {
        id -> Integer,
        small_task_name -> Nullable<Varchar>,
        task_id -> Integer,
    }
}

table! {
    task (id) {
        id -> Integer,
        task_name -> Varchar,
        dead_line -> Timestamp,
        user_id -> Integer,
    }
}

table! {
    user (id) {
        id -> Integer,
        user_name -> Varchar,
        image_url -> Varchar,
    }
}

joinable!(gohobi -> task (task_id));
joinable!(gohobi -> user (from_user_id));
joinable!(small_task -> task (task_id));
joinable!(task -> user (user_id));

allow_tables_to_appear_in_same_query!(friend, gohobi, small_task, task, user,);
