use crate::database::{TodoDB, UserId};
use crate::routes::TodoAppError;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpRequest, HttpResponse, HttpResponseBuilder};
use chrono::NaiveDateTime;
use dotenv;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};

// file for sql table creation
// ../../../../../database/init.sql
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: String,
    pub deleted_at: Option<NaiveDateTime>,
    pub token: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Default::default(),
            username: Default::default(),
            password: Default::default(),
            deleted_at: Default::default(),
            token: Default::default(),
        }
    }
}

// info returned from database create_user
#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: UserId,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
    data: UserInfo,
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

pub async fn create_user(
    body: web::Json<LoginInfo>,
    db: web::Data<TodoDB>,
) -> Result<HttpResponse, Error> {

    let new_token = create_token(&body.username).unwrap();
    let hashed_password = hash(&body.password, DEFAULT_COST).unwrap();
    let new_user = db
        .create_user(&body.username, &hashed_password, &new_token)
        .await?;
    // getDefaultTasks and insertTask(s)
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

pub async fn login(
    body: web::Json<LoginInfo>,
    db: web::Data<TodoDB>,
) -> Result<HttpResponse, TodoAppError> {

    let result = db.get_by_username(&body.username).await;
    if let Some(user) = result {
        let result = verify(&body.password, &user.password);
        if let Ok(valid) = result {
            if valid {
                // addTokenToUser
                let login_token = create_token(&body.username).unwrap();
                db.add_token_to_user(&login_token, user.id).await?;
                return Ok(HttpResponse::Ok().json(LoginResponse { data: user }));
            }
        }
    }
    Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
        .body("incorrect username or password"))
}

// get user from db, compare passwords, create login token
// return user or error 500
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

pub async fn logout(
    req: HttpRequest,
    db: web::Data<TodoDB>,
) -> Result<HttpResponse, TodoAppError> {
    if let Some(user) = db.authenticate(&req).await {
        let row_count = db.find_and_remove_token(&user.token).await?;
        if row_count == 1 {
            let response = MessageResponse {
                message: "user logged out".to_string(),
            };
            return Ok(HttpResponse::Ok().json(response));
        }
    }
    Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
        .body("user not logged in or some other error"))
}


fn create_token(username: &str) -> Result<String, TodoAppError> {
    let secret = dotenv::var("JWT_SECRET");
    match secret {
        Ok(s) => {
            // this needs to be done once somewhere idk where
            let encoding_key = &EncodingKey::from_secret(s.as_bytes());
            let token = encode(&Header::default(), &username, encoding_key).unwrap();
            Ok(token)
        }
        Err(_e) => {
            return Err(TodoAppError {
                name: "could not get secrect from env".to_string(),
            });
        }
    }
}


