use actix_web::http::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::todo::Todo;
use crate::models::user::User;
use crate::models::schema::todos::dsl::*;
use crate::models::schema::users::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,

}   

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            done: Some(false),
            ..todo
        };
        diesel::insert_into(todos)
            .values(&todo)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(todo)
    }

    pub fn create_user(&self, user: User) -> Result<User, Error> {
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            
            ..user
        };
        diesel::insert_into(users)
            .values(&user)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new user");
        Ok(user)
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        todos.load::<Todo>(&mut self.pool.get().unwrap()).expect("Error loading all todos")
    }

    pub fn get_users(&self) -> Vec<User> {
        users.load::<User>(&mut self.pool.get().unwrap()).expect("Error loading all users")
    }
}

