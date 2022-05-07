use actix_web::{web, Error, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use crate::routes::errors::TodoAppError;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    token: String,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
    data: User,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    data: User,
}

#[derive(Serialize, Deserialize)]
struct MessageResponse {
    message: String,
}

/*
# create user
## route: "/"

curl -X POST \
localhost:3010/api/v1/users \
-H "Content-Type: application/json" \
--data '{ "username": "woodroww", "password": "myfancypass" }'

### response:
{
    "data": {
        "id": 3,
        "username": "woodroww",
        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc1ODYzfQ.8CFt61SbF0J7QxpVTYSzfatIrWaAUM8CK_iedXzTjqo"
    }
}
*/

// Create user, create token from username, insert default tasks
// return new user

pub async fn create_user(body: web::Json<LoginInfo>) -> Result<HttpResponse, Error> {
    let new_id = 1;
    let new_token = "createusersjHdsewROUirwe".to_string();
    let new_user = User {
        id: new_id,
        token: new_token,
        username: body.username.clone(),
    };
    let response = CreateUserResponse { data: new_user };
    Ok(HttpResponse::Ok().json(response))
}

/*
# login
## route: "/login" 

curl -X POST \
localhost:3010/api/v1/users/login \
-H "Content-Type: application/json" \
--data '{ "username": "woodroww", "password": "myfancypass" }'

### response:
(with different token than from the creation request)
{
	"data": {
		"id": 3,
		"username": "woodroww",
		"token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE"
	}
}
*/
// get user from db, compare passwords, create login token
// return user or error 500

pub async fn login(body: web::Json<LoginInfo>) -> Result<HttpResponse, Error> {
    let new_id = 1;
    let new_token = "loginsjHdsewROUirwe".to_string();
    let new_user = User {
        id: new_id,
        token: new_token,
        username: body.username.clone(),
    };
    let response = LoginResponse { data: new_user };
    Ok(HttpResponse::Ok().json(response))
}

/*
# logout
## route: "/logout"

curl -X POST \
localhost:3010/api/v1/users/logout \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc2MjQxfQ.dFoUWNAMpiiyXC2lKDsU_tZ88Kvb-lIFOf9_8QEzg9E"

### response:
{"message":"user logged out"}
*/

// find and remove token from db
// return message or error 500

pub async fn logout(req: HttpRequest) -> Result<HttpResponse, TodoAppError> {
    let token = req.headers().get("x-auth-token");
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            println!("we have token: {}", token_string);
        } else {
            return Err(TodoAppError { name: "no x-auth-token string error" });
        }
    } else {
        return Err(TodoAppError { name: "no x-auth-token error" });
    }
    let response = MessageResponse { message: "user logged out".to_string() };
    Ok(HttpResponse::Ok().json(response))
}














