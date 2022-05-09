use crate::database::{TodoDB, UserId};
use crate::routes::errors::TodoAppError;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpRequest, HttpResponse, HttpResponseBuilder};
use chrono::NaiveDateTime;
use dotenv;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};

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

// info returned from db_create_user
#[derive(Serialize, Deserialize)]
pub struct UserCreatedInfo {
    pub id: UserId,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
    data: UserCreatedInfo,
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
        .db_create_user(&body.username, &hashed_password, &new_token)
        .await?;
    let response = CreateUserResponse { data: new_user };
    Ok(HttpResponse::Ok().json(response))
}
/*
use bcrypt::{hash, verify, DEFAULT_COST};
let hashed = hash("hunter2", DEFAULT_COST).unwrap();
let valid = verify("hunter2", &hashed).unwrap();
println!("{:?}", valid);
*/
pub async fn login(
    body: web::Json<LoginInfo>,
    db: web::Data<TodoDB>,
) -> Result<HttpResponse, TodoAppError> {

    let result = db.db_get_by_username(&body.username).await;
    if let Some(user) = result {
        println!("we do have a user {}", &body.username);
        let result = verify(&body.password, &user.password);
        if let Ok(valid) = result {
            println!("we have an Ok from bcrypt");
            if valid {
                println!("we have a valid verify");
                return Ok(HttpResponse::Ok().json(LoginResponse { data: user }));
            }
        } else {
            let err = result.err().unwrap();
            println!("error from verify {}", err.to_string());
        }
    }
    Ok(HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
        .body("incorrect username or password"))
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
            return Err(TodoAppError {
                name: "no x-auth-token string error".to_string(),
            });
        }
    } else {
        return Err(TodoAppError {
            name: "no x-auth-token error".to_string(),
        });
    }
    let response = MessageResponse {
        message: "user logged out".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

use jsonwebtoken::{encode, EncodingKey, Header};

#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
    username: String,
}
// from js return jwt.sign(data, jwtSecret);
fn create_token(username: &str) -> Result<String, TodoAppError> {
    let secret = dotenv::var("JWT_SECRET");
    match secret {
        Ok(s) => {
            // this needs to be done once somewhere idk where
            let encoding_key = &EncodingKey::from_secret(s.as_bytes());
            //let data = UserClaims { username: username.to_string() };
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


mod test {

    use std::process::Command;

    /*
    curl -X POST \
    localhost:3010/api/v1/users \
    -H "Content-Type: application/json" \
    --data '{ "username": "woodroww", "password": "myfancypass" }'
    */

    #[test]
    fn create_user() {
        let output = Command::new("curl")
            .arg("-X")
            .arg("POST")
            .arg("localhost:3010/api/v1/users")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("--data")
            .arg("{ \"username\": \"woodroww\", \"password\": \"myfancypass\" }")
            .output()
            .expect("failure");
        let output_str = String::from_utf8(output.stdout).unwrap();
        println!("output {}", output_str);
        // here id and token are going to be different each time called
        // as we are creating a new user
        //let base_expected = r#"{"data":{"id":3,"username":"woodroww","token":""}}"#;

        assert!(output_str.len() > 0);
    }

    /*
    curl -X POST \
    localhost:3010/api/v1/users/login \
    -H "Content-Type: application/json" \
    --data '{ "username": "woodroww", "password": "myfancypass" }'
    */

    #[test]
    fn login() {
        let output = Command::new("curl")
            .arg("-X")
            .arg("POST")
            .arg("localhost:3010/api/v1/users/login")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("--data")
            .arg("{ \"username\": \"woodroww\", \"password\": \"myfancypass\" }")
            .output()
            .expect("failure");
        let output_str = String::from_utf8(output.stdout).unwrap();
        println!("output {}", output_str);
        let expected = r#"{"data":{"id":2,"username":"woodroww","password":"myfancypass","deleted_at":null,"token":"createuserToken"}}"#;
        assert_eq!(output_str, expected.to_string());
    }
}
