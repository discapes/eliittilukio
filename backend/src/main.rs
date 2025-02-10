use axum::{routing::get, routing::post, Router};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_error;
mod user_api;
mod util;

async fn get_db() -> SqlitePool {
    let db_connection_str: &str = &std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://eliittilukio.sqlite3".to_string());

    if !Sqlite::database_exists(db_connection_str)
        .await
        .unwrap_or(false)
    {
        println!("Creating database {}", db_connection_str);
        match Sqlite::create_database(db_connection_str).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(db_connection_str).await.unwrap();
    db
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Hello world!");
    let db = get_db().await;

    let app = Router::new()
        .route("/create_user", post(user_api::create_user))
        .route("/login", post(user_api::login))
        .route("/update_score", post(user_api::update_score))
        //.route("/send_feedback", post(user_api::send_feedback))
        .route("/list_users", get(user_api::list_users))
        .route("/average_score", get(user_api::average_score))
        .route("/me", get(user_api::me))
        .route("/logout", post(user_api::logout))
        .with_state(db)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
