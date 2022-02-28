use axum::{
    extract::{Form, Path},
    routing::{get, post, MethodRouter},
    Json, Router, http::Method,
};
use db_layer::db::prelude::*;
pub use db_layer::db::RunQueryDsl;
pub use db_layer::db::{delete, insert_into, ConnectionManager, Pool, SqliteConnection};
use db_layer::todo::Todo;
use db_layer::{
    db::prelude::QueryDsl,
    schema::todos::dsl::*,
    todo::{TodoForm, TodoNew},
};
use serde_json::json;
use tower_http::cors::{CorsLayer, Any};

pub fn get_router(pool_db: Pool<ConnectionManager<SqliteConnection>>) -> Router{
    Router::new()
    .merge(todos_list(pool_db.clone()))
    .merge(delete_todo(pool_db.clone()))
    .merge(new_todo(pool_db.clone()))
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(vec![Method::GET, Method::POST, Method::DELETE]),
    )
}

fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new()
    .route(path, method_router)
}

#[tracing::instrument(skip_all)]
fn todos_list(pool: Pool<ConnectionManager<SqliteConnection>>) -> Router {
    route(
        "/todos",
        get(|| async move {
            tracing::info!("Todo list route entered!");
            let db_connection = pool.get().unwrap();
            let all_todos = todos.load::<Todo>(&db_connection).unwrap();
            Json(all_todos)
        }),
    )
}

#[tracing::instrument(skip_all)]
fn new_todo(pool: Pool<ConnectionManager<SqliteConnection>>) -> Router {
    route(
        "/new-todo",
        post(|Form(input): Form<TodoForm>| async move {
            tracing::info!("New todo route entered!");
            let db_connection = pool.get().unwrap();
            let new_todo = TodoNew {
                descr: input.descr,
                date_created: format!("{}", chrono::Local::now().naive_local()),
            };
            insert_into(todos)
                .values(&new_todo)
                .execute(&db_connection)
                .expect("Error");
            let result: Todo = todos.order(id.desc()).first(&db_connection).unwrap();

            tracing::info!(
                "todo with descr {} and id {} inserted",
                result.descr,
                result.id
            );

            Json(result)
        }),
    )
}

#[tracing::instrument(skip_all)]
fn delete_todo(pool: Pool<ConnectionManager<SqliteConnection>>) -> Router {
    route(
        "/delete-todo/:todo_id",
        axum::routing::delete(|Path(todo_id): Path<i32>| async move {
            tracing::info!("Delete todo route entered!");
            let db_connection = pool.get();

            match db_connection {
                Ok(db_connection) => delete(todos.filter(id.eq(todo_id)))
                    .execute(&db_connection)
                    .map_err(|err| {
                        tracing::error!("{err}");
                        return axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    })
                    .map(|o| Json(json!({ "result": format!("todo with id {o} deleted") }))),
                Err(err) => {
                    tracing::error!("{err}");
                    Err(axum::http::StatusCode::SERVICE_UNAVAILABLE)
                }
            }
        }),
    )
}
