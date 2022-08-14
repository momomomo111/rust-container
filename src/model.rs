use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct User {
    // idはauto_incrementなので不要
    pub email: String,
}
