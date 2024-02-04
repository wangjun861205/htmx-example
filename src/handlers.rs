use crate::models::{Todo, TodoCreate};
use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Form, Json},
    HttpResponse, Result,
};
use chrono::Utc;
use sqlx::{query, query_as, query_scalar, PgPool};
use tera::{Context, Tera};

pub async fn todos(pool: Data<PgPool>) -> Result<HttpResponse> {
    let ts = query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(pool.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;
    let t = Tera::new("templates/**/*.html").unwrap();
    let mut context = Context::new();
    context.insert("todos", &ts);
    let html = t
        .render("todo-list.html", &context)
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn create_todo(
    pool: Data<PgPool>,
    Form(TodoCreate {
        title,
        description,
        start_time,
    }): Form<TodoCreate>,
) -> Result<HttpResponse> {
    query_scalar!(
        "INSERT INTO todos (title, description, start_time) VALUES ($1, $2, $3) RETURNING id",
        title,
        description,
        start_time,
    )
    .fetch_one(pool.as_ref())
    .await
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok()
        .insert_header(("HX-Redirect", "/web/index.html"))
        .finish())
}
