mod routes;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/v1")
            .route("/users", web::post().to(routes::users::create_user))
            .route("/users/login", web::post().to(routes::users::login))
            .route("/users/logout", web::post().to(routes::users::logout))
            .route("/tasks", web::post().to(routes::tasks::create_task))
            .route("/tasks/{id}", web::get().to(routes::tasks::get_task_id))
            .route("/tasks/{id}/completed", web::put().to(routes::tasks::set_task_completed))
            .route("/tasks/{id}/uncompleted", web::put().to(routes::tasks::set_task_uncompleted))
        )
    })
    .bind(("127.0.0.1", 3010))?
    .run()
    .await
}


