use axum::{
    routing::{get, post, delete, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use sqlx::mysql::MySqlPoolOptions;

mod handlers;
mod model;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(Any);

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url) 
        .await {
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

    println!("🚀 Server started successfully");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handlers::health_checker_handler))
        .route("/health_checker_handler", get(handlers::health_checker_handler))
        .route("/api/mytable", get(handlers::getall_mytable))
        .route("/api/mytable", post(handlers::create_mytable))
        .route("/api/mytable/:nomer", get(handlers::get_mytable))
        .route("/api/mytable/:nomer", delete(handlers::delete_mytable))
        .route("/api/mytable/:nomer", put(handlers::update_mytable))
        .with_state(pool)
        .layer(cors);
    
    println!("Listening on port {}", "0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

