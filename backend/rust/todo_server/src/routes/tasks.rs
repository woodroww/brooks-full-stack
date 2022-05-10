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

// file for sql table creation
// ../../../../../database/init.sql
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

/*
# create a task
## route: "/"

curl -X POST \
localhost:3010/api/v1/tasks \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc2OTkyfQ.iEgWdomqYA3SkFZOiQmSvQPFLSW4kfsHVxA9p-WN8KA" \
-H "Content-Type: application/json" \
--data '{ "title": "Curl is fun", "description": "typing and stuff in the terminal" }'

### response:
{
    "data": {
        "id": 8,
        "priority": null,
        "title": "Curl is fun",
        "completed_at": null,
        "description": "typing and stuff in the terminal"
    }
}
*/


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

/*
# get a task
## route: "/:taskId"

curl -X GET \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE" \
localhost:3010/api/v1/tasks/8

### response:
{
    "data": {
        "id": 8,
        "priority": null,
        "title": "Curl is fun",
        "completed_at": null,
        "description": "typing and stuff in the terminal",
        "deleted_at": null,
        "user_id": 3,
        "is_default": false
    }
}
*/

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

/*
# set task with id as completed
## route: "/:taskId/completed"

curl -X PUT \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE" \
localhost:3010/api/v1/tasks/8/completed

### response:
OK
*/

pub async fn set_task_completed(
    req: HttpRequest,
    db: web::Data<TodoDB>,
    id: web::Path<TaskId>,
) -> Result<HttpResponse, TodoAppError> {
    if let Some(user) = db.authenticate(&req).await {
        println!("we have authentication");
        if db.mark_completed(user.id, *id).await {
            println!("we have mark_completed");
            return Ok(HttpResponse::Ok().body(format!("OK you completed task {}", id.into_inner())));
        }
    }
    Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST).body("error"))
}

/*
# set task with id as uncompleted
## route: "/:taskId/uncompleted"

curl -X PUT \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE" \
localhost:3010/api/v1/tasks/8/uncompleted

### response:
OK
*/

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
