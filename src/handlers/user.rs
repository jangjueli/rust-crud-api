use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::{User, CreateUser};

// GET user list
pub async fn list_users(db_pool: web::Data<PgPool>) -> impl Responder {
    let users = sqlx::query_as!(User, "SELECT id, name FROM users ORDER BY id ASC")
        .fetch_all(db_pool.get_ref())
        .await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// GET user from id
pub async fn get_user(db_pool: web::Data<PgPool>,user_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as!(User, "SELECT id, name FROM users WHERE id = $1", *user_id)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            eprintln!("Database error: {}",e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// POST create user
pub async fn create_user(db_pool: web::Data<PgPool>,new_user: web::Json<CreateUser>) -> impl Responder {
    let result = sqlx::query!("INSERT INTO users (name) VALUES ($1) RETURNING id",new_user.name)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(User {
            id: record.id,
            name: new_user.name.clone(),
        }),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// DELETE user
pub async fn delete_user(db_pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1 RETURNING id", *user_id)
        .fetch_one(db_pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body(format!("User {} deleted", user_id)),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// UPDATE user
pub async fn update_user(db_pool: web::Data<PgPool>,user_id: web::Path<i32>,updated_user: web::Json<CreateUser>) -> impl Responder {
    let result = sqlx::query!("UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name",updated_user.name,*user_id)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(User {
            id: record.id,
            name: record.name,
        }),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
