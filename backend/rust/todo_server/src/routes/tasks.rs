// TODO
// updateTask - update task like user has changed the text
// softDeleteTask - set deleted_at to now in db
// getDefaultTasks - like for a new user to have default tasks as examples

use crate::database::{TaskId, TodoDB, UserId};
use crate::routes::TodoAppError;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, HttpResponseBuilder};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: TaskId,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<NaiveDateTime>,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
struct TaskResponse {
    data: TaskInfo,
}

#[derive(Serialize, Deserialize)]
struct TaskListResponse {
    data: Vec<TaskInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<NaiveDateTime>,
    pub description: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub user_id: UserId,
    pub is_default: bool,
}

pub async fn create_task(
    req: HttpRequest,
    body: web::Json<CreateTaskRequest>,
    db: web::Data<TodoDB>,
) -> Result<HttpResponse, TodoAppError> {
    if let Some(user) = db.authenticate(&req).await {
        let create_request = CreateTaskRequest {
            title: body.title.clone(),
            description: body.description.clone(),
        };
        let create_response = db.insert_task(&create_request, user.id).await;
        if let Some(info) = create_response {
            return Ok(HttpResponse::Ok().json(TaskResponse { data: info }));
        } else {
            return Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                .body("something wrong with db returning inserted task"));
        }
    }
    Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body("invalid token"))
}

pub async fn get_task_id(
    req: HttpRequest,
    db: web::Data<TodoDB>,
    id: web::Path<i32>,
) -> Result<HttpResponse, TodoAppError> {
    if let Some(user) = db.authenticate(&req).await {
        let task_id = id.into_inner();
        let task = db.get_task(user.id, task_id).await;
        if let Some(t) = task {
            let info = TaskInfo {
                id: t.id,
                priority: t.priority,
                title: t.title,
                completed_at: t.completed_at,
                description: t.description,
            };
            return Ok(HttpResponse::Ok().json(TaskResponse { data: info }));
        } else {
            return Ok(
                HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body("no such task")
            );
        }
    }
    return Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body("invalid token"));
}

pub async fn get_all_tasks(
    req: HttpRequest,
    db: web::Data<TodoDB>,
) -> Result<HttpResponse, TodoAppError> {

    if let Some(user) = db.authenticate(&req).await {
        let result = db.get_all_tasks(user.id).await;
        if let Some(tasks) = result {
            return Ok(HttpResponse::Ok().json(TaskListResponse { data: tasks }));
        }
    }
    return Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body("invalid token"));
}

pub async fn set_task_completed(
    req: HttpRequest,
    db: web::Data<TodoDB>,
    id: web::Path<TaskId>,
) -> Result<HttpResponse, TodoAppError> {
    if let Some(user) = db.authenticate(&req).await {
        if db.mark_completed(user.id, *id).await {
            return Ok(HttpResponse::Ok().body(format!("OK you completed task {}", id.into_inner())));
        }
    }
    Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body("error"))
}

pub async fn set_task_uncompleted(
    req: HttpRequest,
    db: web::Data<TodoDB>,
    id: web::Path<TaskId>,
) -> Result<HttpResponse, TodoAppError> {
    if let Some(user) = db.authenticate(&req).await {
        if db.mark_uncompleted(user.id, *id).await {
            return Ok(HttpResponse::Ok().body(format!("OK you un-completed task {}", id.into_inner())));
        }
    }
    Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body("error"))
}




