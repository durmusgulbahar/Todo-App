
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::models::schema::users)]
pub struct User {
    #[serde(default)]
    pub id: String,
    pub username: String,
    pub password: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
