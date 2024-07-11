use actix_web::{
    delete, get, patch, post,
    web::{scope, Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};

use serde_json::json;
use uuid::Uuid;

use crate::{
    model::TaskModel,
    schema::{CreateTaskSchema, FilterOptions, UpdateTaskSchema},
    AppState,
};

#[get("/healthcheck")]
async fn health_check() -> impl Responder {
    const MESSAGE: &str = "Healt check: API is up and running smoothly!";
    HttpResponse::Ok().json(json!({ "status": "success", "message": MESSAGE }))
}

#[post("/tasks")]
async fn create_task(body: Json<CreateTaskSchema>, data: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks (title, content) VALUES ($1, $2) RETURNING * ",
        body.title.to_string(),
        body.content.to_string(),
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(task) => {
            let note_response = json!({
                "status": "success",
                "data": task
            });
            HttpResponse::Created().json(note_response)
        }
        Err(err) => HttpResponse::InternalServerError()
            .json(json!({ "status": "error", "message": format!(" {:?}", err)})),
    }
}

#[get("/tasks")]
async fn get_all_tasks(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(tasks) => {
            let json_response = json!({
                "status": "success",
                "result": tasks.len(),
                "tasks": tasks
            });

            HttpResponse::Ok().json(json_response)
        }
        Err(err) => HttpResponse::InternalServerError()
            .json(json!({ "status": "error", "message": format!(" {:?}", err)})),
    }
}

#[get("/tasks/{id}")]
async fn get_task_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(task) => {
            let task_note = json!({
                "status": "success",
                "task": task
            });
            HttpResponse::Ok().json(task_note)
        }
        Err(err) => HttpResponse::InternalServerError()
            .json(json!({ "status": "error", "message": format!(" {:?}", err)})),
    }
}

#[delete("/tasks/{id}")]
async fn delete_task_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(TaskModel, "DELETE FROM tasks WHERE id = $1", task_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError()
            .json(json!({ "status": "error", "message": format!(" {:?}", err)})),
    }
}

#[patch("/tasks/{id}")]
async fn update_task_by_id(
    path: Path<Uuid>,
    body: Json<UpdateTaskSchema>,
    data: Data<AppState>,
) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(task) => {
            match sqlx::query_as!(
                TaskModel,
                "UPDATE tasks SET title = $1, content = $2 WHERE id = $3 RETURNING *",
                body.title.to_owned().unwrap_or(task.title),
                body.content.to_owned().unwrap_or(task.content),
                task_id
            )
            .fetch_one(&data.db)
            .await
            {
                Ok(task) => {
                    let task_response = json!({
                        "status": "success",
                        "task": task,
                    });
                    HttpResponse::Ok().json(task_response)
                }
                Err(err) => HttpResponse::InternalServerError()
                    .json(json!({ "status": "error", "message": format!(" {:?}", err)})),
            }
        }
        Err(err) => HttpResponse::InternalServerError()
            .json(json!({ "status": "error", "message": format!(" {:?}", err)})),
    }
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(health_check)
        .service(create_task)
        .service(get_all_tasks)
        .service(get_task_by_id)
        .service(delete_task_by_id)
        .service(update_task_by_id)
        .service(update_task_by_id);
    conf.service(scope);
}
