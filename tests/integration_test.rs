use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
    routing::{get, post, put, delete},
    middleware,
};

use tower::ServiceExt;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use api_rustone::{
    model::model::{CreateUserRequest, LoginRequest, CreatePostRequest},
    handlers::{
        handlers::root,
        auth_handlers::{register_user, login_user, get_profile, update_profile},
        post_handlers::{create_post, get_post, get_user_posts, get_all_posts, update_post, delete_post},
    },
    helpers::middleware::{auth_middleware, optional_auth_middleware},
    db::sql_db::get_sql_client,
};

// Test app setup
async fn create_test_app() -> Router {
    dotenv::dotenv().ok();
    
    // Database setup
    let sql_db = match get_sql_client().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to connect to SQLite database: {}", e);
            panic!("Database connection failed");
        }
    };
    
    let pool = Arc::new(sql_db.get_pool().clone());
    
    // CORS setup
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Route setup
    Router::new()
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
        .with_state(pool)
}

#[tokio::test]
async fn test_register_user() {
    let app = create_test_app().await;
    
    let user_data = CreateUserRequest {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        password: "TestPass123".to_string(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&user_data).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_login_user() {
    let app = create_test_app().await;
    
    // Register user
    let user_data = CreateUserRequest {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        password: "TestPass123".to_string(),
    };

    let register_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&user_data).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);
    
    // Login user
    let login_data = LoginRequest {
        email: "test@example.com".to_string(),
        password: "TestPass123".to_string(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&login_data).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_post() {
    let app = create_test_app().await;
    
    let post_data = CreatePostRequest {
        title: "Test Post".to_string(),
        content: "This is a test post content.".to_string(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/posts")
                .header("content-type", "application/json")
                .header("authorization", "Bearer test-token")
                .body(Body::from(serde_json::to_string(&post_data).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Check unauthorized
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
} 