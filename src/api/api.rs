use actix_web::{web, get};
use actix_web::{web::{
    Data,
    Json,
}, post, HttpResponse};
use crate::{models::todo::Todo, models::user::User,repository::database::Database};

#[post("/todos")]
pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(db: web::Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)
}

#[post("/users")]
pub async fn create_user(db: Data<Database>, new_user: Json<User>) -> HttpResponse {
    let user= db.create_user(new_user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_users(db: web::Data<Database>) -> HttpResponse {
    let users = db.get_users();
    HttpResponse::Ok().json(users)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(
            "/TodoApp/api"
        ).service(create_todo)
        .service(get_todos)
        .service(create_user)
        .service(get_users)
        
    );
}