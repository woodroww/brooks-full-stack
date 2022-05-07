/* Routes
/Users/matt/external_code/BrooksYew/brooks-full-stack/requests.md

./user/
./user/login
./user/logout
./tasks/
./tasks/:taskId
./tasks/:taskId/completed
./tasks/:taskId/uncompleted
*/

mod routes;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/v1/users")
            .service(routes::users::create_user)
            .service(routes::users::login)
            .service(routes::users::logout)
        )
    })
    .bind(("127.0.0.1", 3010))?
    .run()
    .await
}


