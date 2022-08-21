use std::time::SystemTime;

use crate::schema::{friend, gohobi, small_task, task, user};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub image_url: String,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "task"]
pub struct Task {
    pub id: i32,
    pub task_name: String,
    pub dead_line: SystemTime,
    pub user_id: i32,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "small_task"]
pub struct SmallTask {
    pub id: i32,
    pub small_task_name: String,
    pub task_id: i32,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "gohobi"]
pub struct Gohobi {
    pub id: i32,
    pub message: String,
    pub from_user_id: i32,
    pub task_id: i32,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "friend"]
pub struct Friend {
    pub owner_id: i32,
    pub friend_name: i32,
}
