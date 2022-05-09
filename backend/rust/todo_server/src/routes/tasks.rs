use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use crate::routes::errors::TodoAppError;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
struct CreateTaskInfo {
    id: u32,
    priority: Option<u32>,
    title: String,
    completed_at: Option<u32>,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct CreateTaskResponse {
    data: CreateTaskInfo,
}

// file for sql table creation 
// ../../../../../database/init.sql
#[derive(Serialize, Deserialize)]
pub struct Task {
    id: i32,
    priority: Option<String>,
    title: String,
    completed_at: Option<DateTime<Utc>>,
    description: String,
    deleted_at: Option<DateTime<Utc>>,
    user_id: i32,
    is_default: bool,
}

#[derive(Serialize, Deserialize)]
struct TaskResponse {
    data: Task,
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

pub async fn create_task(req: HttpRequest) -> Result<HttpResponse, TodoAppError> {
    let token = req.headers().get("x-auth-token");
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            println!("we have token: {}", token_string);
        } else {
            return Err(TodoAppError { name: "invalid x-auth-token error".to_string() });
        }
    } else {
        return Err(TodoAppError { name: "invalid x-auth-token error".to_string() });
    }
    let title = "Title".to_string();
    let description = "Description".to_string();

    let create_info = CreateTaskInfo {
        id: 0,
        priority: None,
        title,
        completed_at: None,
        description,
    };

    Ok(HttpResponse::Ok().json(CreateTaskResponse { data: create_info }))
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

pub async fn get_task_id(req: HttpRequest, id: web::Path<i32>) -> Result<HttpResponse, TodoAppError> {
    let token = req.headers().get("x-auth-token");
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            println!("we have token: {}", token_string);
        } else {
            return Err(TodoAppError { name: "invalid x-auth-token error".to_string() });
        }
    } else {
        return Err(TodoAppError { name: "invalid x-auth-token error".to_string() });
    }

    let find_id = id.into_inner();

    let task = Task {
        id: find_id,
        priority: None,
        title: "what".to_string(),
        completed_at: None,
        description: "task description".to_string(),
        deleted_at: None,
        user_id: 123,
        is_default: false,
    };
    Ok(HttpResponse::Ok().json(TaskResponse { data: task }))
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

pub async fn set_task_completed(req: HttpRequest, id: web::Path<u32>) -> Result<HttpResponse, TodoAppError> {
    let token = req.headers().get("x-auth-token");
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            println!("we have token: {}", token_string);
        } else {
            return Err(TodoAppError { name: "invalid x-auth-token error".to_string() });
        }
    } else {
        return Err(TodoAppError { name: "invalid x-auth-token error".to_string() });
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

pub async fn set_task_uncompleted(req: HttpRequest, id: web::Path<u32>) -> Result<HttpResponse, TodoAppError> {
    let token = req.headers().get("x-auth-token");
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            println!("we have token: {}", token_string);
        } else {
            return Err(TodoAppError { name: "invalid x-auth-token error".to_string() });
        }
    } else {
        return Err(TodoAppError { name: "invalid x-auth-token error".to_string() });
    }
    Ok(HttpResponse::Ok().body(format!("OK you un-completed task {}", id.into_inner())))
}

