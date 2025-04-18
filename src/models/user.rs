use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone,Serialize, Deserialize, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}
