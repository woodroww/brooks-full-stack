use crate::routes::TodoAppError;
use crate::database::{TodoDB, UserId, TaskId};
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
    let token = req.headers().get("x-auth-token");
    let user_id;
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            let authentication_result = db.authenticate(token_string).await;
            if let Some(user) = authentication_result {
                user_id = user.id;
            } else {
                return Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                    .body("invalid token"))
            }
        } else {
            return Err(TodoAppError {
                name: "invalid x-auth-token error".to_string(),
            });
        }
    } else {
        return Err(TodoAppError {
            name: "invalid x-auth-token error".to_string(),
        });
    }

    let create_request = CreateTaskRequest {
        title: body.title.clone(),
        description: body.description.clone(),
    };
    let create_response = db.db_insert_task(&create_request, user_id).await;
    if let Some(info) = create_response {
        Ok(HttpResponse::Ok().json(TaskResponse { data: info }))
    } else {
        Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
            .body("something wrong with db returning inserted task"))
    }
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
    let token = req.headers().get("x-auth-token");
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            if let Some(user) = db.authenticate(token_string).await {
                let task_id = id.into_inner();
                let task = db.db_get_task(user.id, task_id).await;
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
                    return Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                        .body("no such task"))
                }
            }
        }
    }
    return Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
        .body("invalid token"))
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
    id: web::Path<u32>,
) -> Result<HttpResponse, TodoAppError> {
    let token = req.headers().get("x-auth-token");
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            println!("we have token: {}", token_string);
        } else {
            return Err(TodoAppError {
                name: "invalid x-auth-token error".to_string(),
            });
        }
    } else {
        return Err(TodoAppError {
            name: "invalid x-auth-token error".to_string(),
        });
    }
    Ok(HttpResponse::Ok().body(format!("OK you completed task {}", id.into_inner())))
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
    id: web::Path<u32>,
) -> Result<HttpResponse, TodoAppError> {
    let token = req.headers().get("x-auth-token");
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            println!("we have token: {}", token_string);
        } else {
            return Err(TodoAppError {
                name: "invalid x-auth-token error".to_string(),
            });
        }
    } else {
        return Err(TodoAppError {
            name: "invalid x-auth-token error".to_string(),
        });
    }
    Ok(HttpResponse::Ok().body(format!("OK you un-completed task {}", id.into_inner())))
}
