use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use dotenv::dotenv;

mod handlers;
use handlers::{
    handlers::{root},
    auth_handlers::{register_user, login_user, get_profile, update_profile},
    post_handlers::{create_post, get_post, get_user_posts, get_all_posts, update_post, delete_post},
};
pub mod model;
pub use model::model::User;

mod db;
use db::sql_db::get_sql_client;

pub mod helpers;
use helpers::middleware::{auth_middleware, optional_auth_middleware};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Database setup
    let sql_db = match get_sql_client().await {
        Ok(db) => {
            println!("Connected to SQLite database successfully!");
            db
        },
        Err(e) => {
            eprintln!("Failed to connect to SQLite database: {}", e);
            std::process::exit(1);
        }
    };
    
    let pool = Arc::new(sql_db.get_pool().clone());
    
    // CORS setup
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Route setup
    let app = Router::new()
        .route("/", get(root))
        
        // Public routes
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .route("/posts", get(get_all_posts))
        .route("/posts/{id}", get(get_post))
        
        // Protected routes
        .route("/auth/profile", get(get_profile))
        .route("/auth/profile", put(update_profile))
        .route("/posts", post(create_post))
        .route("/posts/my", get(get_user_posts))
        .route("/posts/{id}", put(update_post))
        .route("/posts/{id}", delete(delete_post))
        
        .layer(cors)
        .layer(middleware::from_fn_with_state(
            pool.clone(),
            |req: axum::extract::Request, next: axum::middleware::Next| async move {
                // Auth middleware
                let path = req.uri().path();
                if path.starts_with("/auth/profile") || 
                   path.starts_with("/posts") && req.method() == "POST" ||
                   path.starts_with("/posts/my") ||
                   (path.starts_with("/posts/") && (req.method() == "PUT" || req.method() == "DELETE")) {
                    auth_middleware(req, next).await
                } else {
                    optional_auth_middleware(req, next).await
                }
            }
        ))
        .with_state(pool);
    
    let socket_address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8081));

    // Server startup
    
    tracing::debug!("Listening on {}", socket_address);
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}




    


