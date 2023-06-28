
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, sql_types::Bool};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::models::schema::todos)]
pub struct Todo {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub done: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub user_id: String,
}



