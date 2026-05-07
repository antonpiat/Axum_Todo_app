pub mod user;
pub mod todo;

use user::models::AuthReq;
use user::handlers::*;
use todo::models::*;
use todo::handlers::*;
use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, get_service, post, put},
};

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        user::handlers::register,
        user::handlers::login,
        user::handlers::logout,
        user::handlers::check_auth,
        todo::handlers::get_list,
        todo::handlers::create_todo,
        todo::handlers::delete_todo,
        todo::handlers::update_todo,
    ),
    components(
        schemas(AuthReq, User, UpdateTodo, Todo, CreateTodo)
    ),
    tags(
        (name = "Todo App", description = "Todo list management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:8080".to_owned());
    let db_url = std::env::var("DB_URL").expect("DB_URL is not set in .env file");
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");
    println!("DB connected successfully");

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Failed to create a TCP listener");

    let router = Router::new()
        .route("/todo", get(get_list).post(create_todo))
        .route("/todo/{todo_id}", put(update_todo).delete(delete_todo))
        .layer(from_fn_with_state(db_pool.clone(), auth_middleware));

    let user_router = Router::new()
        .route("/users", post(register))
        .route("/logout", post(logout))
        .route("/login", post(login))
        .route("/check_auth", get(check_auth))
        .merge(router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .fallback_service(get_service(ServeDir::new("../frontend/out")))
        .with_state(db_pool);

    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, user_router)
        .await
        .expect("Could not start the server");
}
