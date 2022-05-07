use actix_web::{post, web, error, Error, HttpResponse, HttpRequest};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

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
struct LoginInfo {
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

#[derive(Debug, Display, Error)]
#[display(fmt = "users error: {}", name)]
struct UsersError {
    name: &'static str
}

// Use default implementation for `error_response()` method
impl error::ResponseError for UsersError {}

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

#[post("/")]
async fn create_user(body: web::Json<LoginInfo>) -> Result<HttpResponse, Error> {
    println!("actix server recieved username: {}", body.username);
    println!("actix server recieved password: {}", body.password);
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
localhost:3010/login \
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

#[post("/login")]
async fn login(body: web::Json<LoginInfo>) -> Result<HttpResponse, Error> {
    println!("actix server recieved username: {}", body.username);
    println!("actix server recieved password: {}", body.password);
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

#[post("/logout")]
async fn logout(req: HttpRequest) -> Result<HttpResponse, UsersError> {
    let token = req.headers().get("x-auth-token");
    if let Some(t) = token {
        if let Some(token_string) = t.to_str().ok() {
            println!("we have token: {}", token_string);
        } else {
            return Err(UsersError { name: "no x-auth-token string error" });
        }
    } else {
        return Err(UsersError { name: "no x-auth-token error" });
    }
    let response = MessageResponse { message: "user logged out".to_string() };
    Ok(HttpResponse::Ok().json(response))
}














