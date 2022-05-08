use actix_web::{App, HttpServer, web};
use todo_server::routes::users as user;
use todo_server::routes::tasks as task;
use todo_server::TodoDBConnectionManager;
use tokio_postgres::{Config, NoTls};
use mobc_postgres::PgConnectionManager;

type Pool = mobc::Pool<TodoDBConnectionManager>;

/*struct AppState {
    db_pool: Pool,
}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut config = Config::new();
    config.host("localhost");
    config.dbname("google_coursera");
    config.user("matt");
    config.port(666);

    let manager = PgConnectionManager::new(config, NoTls);
    let pool = Pool::builder().max_open(10).build(TodoDBConnectionManager);

    HttpServer::new(move || {
        App::new()
            //.app_data(pool.clone())
            .service( web::scope("/api/v1")
            .route("/users", web::post().to(user::create_user))
            .route("/users/login", web::post().to(user::login))
            .route("/users/logout", web::post().to(user::logout))
            .route("/tasks", web::post().to(task::create_task))
            .route("/tasks/{id}", web::get().to(task::get_task_id))
            .route("/tasks/{id}/completed", web::put().to(task::set_task_completed))
            .route("/tasks/{id}/uncompleted", web::put().to(task::set_task_uncompleted))
        )
    })
    .bind(("127.0.0.1", 3010))?
    .run()
    .await
}

// /Users/matt/Documents/Programming/rust/postgres-test/src/main.rs
