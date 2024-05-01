use crate::models::User;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/users")]
async fn get_users(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&**pool)
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/users")]
async fn create_user(user: web::Json<User>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
        user.name,
        user.email
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record.id),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/users/{user_id}")]
async fn get_user(user_id: web::Path<i32>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    println!("user_id :{:?}", user_id);
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", *user_id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
