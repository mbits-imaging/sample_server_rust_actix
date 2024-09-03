use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{db, DbPool};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

#[post("/users")]
async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    if let Ok(mut conn) = pool.get() {
        match db::insert_new_user(&mut conn, &user.name) {
            Ok(user) => HttpResponse::Created().json(&user),
            Err(e) => error::ErrorInternalServerError(e).into(),
        }
    } else {
        error::ErrorInternalServerError("couldn't get db connection from pool").into()
    }
}

#[get("/users")]
async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    if let Ok(mut conn) = pool.get() {
        match db::get_users(&mut conn) {
            Ok(users) => HttpResponse::Ok().json(&users),
            Err(e) => error::ErrorInternalServerError(e).into(),
        }
    } else {
        error::ErrorInternalServerError("couldn't get db connection from pool").into()
    }
}

#[get("/users/{id}")]
async fn get_user_by_id(pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    if let Ok(mut conn) = pool.get() {
        match db::get_user_by_id(&mut conn, id.into_inner()) {
            Ok(user) => match user {
                Some(user) => HttpResponse::Ok().json(&user),
                None => HttpResponse::NotFound().finish(),
            },
            Err(e) => error::ErrorInternalServerError(e).into(),
        }
    } else {
        error::ErrorInternalServerError("couldn't get db connection from pool").into()
    }
}

#[put("/users/{id}")]
async fn update_user_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    if let Ok(mut conn) = pool.get() {
        match db::update_user_by_id(&mut conn, id.into_inner(), &user.name) {
            Ok(user) => match user {
                Some(user) => HttpResponse::Ok().json(&user),
                None => HttpResponse::NotFound().finish(),
            },
            Err(e) => error::ErrorInternalServerError(e).into(),
        }
    } else {
        error::ErrorInternalServerError("couldn't get db connection from pool").into()
    }
}

#[delete("/users/{id}")]
async fn delete_user_by_id(pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    if let Ok(mut conn) = pool.get() {
        match db::delete_user_by_id(&mut conn, id.into_inner()) {
            Ok(user) => match user {
                Some(_) => HttpResponse::NoContent().finish(),
                None => HttpResponse::NotFound().finish(),
            },
            Err(e) => error::ErrorInternalServerError(e).into(),
        }
    } else {
        error::ErrorInternalServerError("couldn't get db connection from pool").into()
    }
}
