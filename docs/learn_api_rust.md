# Building REST APIs with Rust and Axum

This guide will walk you through the process of creating REST APIs in Rust using the Axum framework. We'll cover everything from project setup to implementing GET, POST, PUT, and DELETE endpoints.

## Table of Contents
- [Building REST APIs with Rust and Axum](#building-rest-apis-with-rust-and-axum)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Getting Started](#getting-started)
  - [Project Setup](#project-setup)
  - [Basic API Structure](#basic-api-structure)
  - [Implementing CRUD Operations](#implementing-crud-operations)
    - [GET Endpoints](#get-endpoints)
    - [POST Endpoints](#post-endpoints)
    - [PUT Endpoints](#put-endpoints)
    - [DELETE Endpoints](#delete-endpoints)
  - [Working with JSON](#working-with-json)
  - [Error Handling](#error-handling)
  - [Authentication](#authentication)
  - [Testing](#testing)
  - [Best Practices](#best-practices)
  - [Conclusion](#conclusion)
  - [Resources](#resources)

## Introduction

Axum is a web framework built on top of Tokio, Hyper, and Tower. It's designed to be modular, 
ergonomic, and fast. Axum is particularly well-suited for building REST APIs because of
its focus on routing, middleware, and extractors.

## Getting Started

First, set up a new Rust project and add Axum as a dependency:

```bash
cargo new my_api
cd my_api
```

Add the following to your `Cargo.toml`:

```toml
[dependencies]
axum = "0.7.2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower = "0.4"
tower-http = { version = "0.4", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## Project Setup

Here's the basic structure of an Axum application:

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user));

    // Run our application
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Request handlers
async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_users() -> &'static str {
    "Get all users"
}

async fn get_user() -> &'static str {
    "Get a specific user"
}

async fn create_user() -> &'static str {
    "Create a user"
}

async fn update_user() -> &'static str {
    "Update a user"
}

async fn delete_user() -> &'static str {
    "Delete a user"
}
```

## Basic API Structure

Axum uses handlers (functions) to process requests. Handlers can be any async function that returns a type that implements `IntoResponse`. Routes are defined using the `Router::route` method, which takes a path and one or more handlers for different HTTP methods.

## Implementing CRUD Operations

### GET Endpoints

For GET requests, you typically extract path parameters or query parameters and return data:

```rust
use axum::{
    extract::{Path, Query},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define your data model
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// GET all users
async fn get_users() -> Json<Vec<User>> {
    // In a real app, you would fetch from a database
    let users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
    ];
    
    Json(users)
}

// GET a specific user by ID
async fn get_user(Path(user_id): Path<u64>) -> Json<Option<User>> {
    // In a real app, you would fetch from a database
    let user = if user_id == 1 {
        Some(User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() })
    } else if user_id == 2 {
        Some(User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() })
    } else {
        None
    };
    
    Json(user)
}

// GET with query parameters
#[derive(Deserialize)]
struct UserQuery {
    name: Option<String>,
    limit: Option<usize>,
}

async fn search_users(Query(params): Query<UserQuery>) -> Json<Vec<User>> {
    let mut users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
    ];
    
    // Filter by name if provided
    if let Some(name) = params.name {
        users = users.into_iter()
            .filter(|user| user.name.contains(&name))
            .collect();
    }
    
    // Apply limit if provided
    if let Some(limit) = params.limit {
        users.truncate(limit);
    }
    
    Json(users)
}
```

### POST Endpoints

For POST requests, you typically extract JSON from the request body:

```rust
// Define the request body structure
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

// Define the response structure
#[derive(Serialize)]
struct CreateUserResponse {
    id: u64,
    name: String,
    email: String,
}

async fn create_user(
    Json(payload): Json<CreateUserRequest>
) -> Json<CreateUserResponse> {
    // In a real app, you would insert into a database
    let user = CreateUserResponse {
        id: 3, // Generated ID
        name: payload.name,
        email: payload.email,
    };
    
    Json(user)
}
```

### PUT Endpoints

PUT requests often combine path parameters with JSON body:

```rust
#[derive(Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
}

async fn update_user(
    Path(user_id): Path<u64>,
    Json(payload): Json<UpdateUserRequest>
) -> Json<User> {
    // In a real app, you would update the database
    // Here we're just simulating an update
    let mut user = User {
        id: user_id,
        name: "Old Name".to_string(),
        email: "old@example.com".to_string(),
    };
    
    if let Some(name) = payload.name {
        user.name = name;
    }
    
    if let Some(email) = payload.email {
        user.email = email;
    }
    
    Json(user)
}
```

### DELETE Endpoints

DELETE requests typically use path parameters:

```rust
async fn delete_user(Path(user_id): Path<u64>) -> Json<HashMap<String, String>> {
    // In a real app, you would delete from the database
    
    let mut response = HashMap::new();
    response.insert("status".to_string(), "success".to_string());
    response.insert("message".to_string(), format!("User with ID {} deleted", user_id));
    
    Json(response)
}
```

## Working with JSON

Axum makes it easy to work with JSON using the `Json` extractor. You define your data structures using Serde's derive macros:

```rust
use serde::{Deserialize, Serialize};
use axum::Json;

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

async fn create_todo(Json(todo): Json<Todo>) -> Json<Todo> {
    // Process the todo
    Json(todo)
}
```

## Error Handling

Error handling in Axum is done by implementing the `IntoResponse` trait for your error types:

```rust
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
enum AppError {
    NotFound,
    InvalidInput(String),
    DatabaseError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Resource not found"),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::InvalidInput(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

// Now you can return errors from your handlers
async fn get_user(Path(user_id): Path<u64>) -> Result<Json<User>, AppError> {
    if user_id == 0 {
        return Err(AppError::InvalidInput("User ID cannot be zero".to_string()));
    }
    
    // Assume this is a database lookup
    let user = find_user(user_id).await?;
    
    Ok(Json(user))
}

async fn find_user(id: u64) -> Result<User, AppError> {
    // Simulating a database lookup
    if id > 100 {
        return Err(AppError::NotFound);
    }
    
    Ok(User {
        id,
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    })
}
```

## Authentication

Here's a simple example of how to implement authentication using middleware:

```rust
use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
};
use tower_http::auth::RequireAuthorizationLayer;

async fn auth_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // Get the authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    // Check if the token is valid
    match auth_header {
        Some(token) if token.starts_with("Bearer ") => {
            let token = &token["Bearer ".len()..];
            // In a real app, you would validate the token
            if token == "valid-token" {
                // If the token is valid, continue to the handler
                Ok(next.run(req).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

// Apply the middleware to specific routes
let app = Router::new()
    .route("/public", get(public_handler))
    .route(
        "/protected",
        get(protected_handler)
            .layer(middleware::from_fn(auth_middleware)),
    );
```

## Testing

Axum provides tools for testing your API:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_get_users() {
        // Build our application with the routes
        let app = Router::new().route("/users", get(get_users));

        // Create a request
        let request = Request::builder()
            .uri("/users")
            .body(Body::empty())
            .unwrap();

        // Process the request and get the response
        let response = app
            .oneshot(request)
            .await
            .unwrap();

        // Check the response
        assert_eq!(response.status(), StatusCode::OK);
        
        // Get the body
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let users: Vec<User> = serde_json::from_slice(&body).unwrap();
        
        // Check the content
        assert!(!users.is_empty());
    }
}
```

## Best Practices

1. **Organize Your Code**: Separate your application into modules (handlers, models, services, etc.)
2. **Use DTOs**: Create separate structs for requests and responses
3. **Implement Proper Validation**: Validate incoming data early
4. **Use Proper Error Handling**: Create custom error types
5. **Add Middleware**: Use middleware for cross-cutting concerns (logging, auth, etc.)
6. **Follow RESTful Conventions**: Use appropriate HTTP methods and status codes
7. **Document Your API**: Consider using OpenAPI/Swagger
8. **Write Tests**: Include unit and integration tests

## Conclusion

This guide covers the basics of building REST APIs with Axum. As you get more comfortable, you can explore more advanced features like:

- WebSockets
- File uploads
- Rate limiting
- Async database integrations
- GraphQL
- CORS support

Remember that the Rust ecosystem is constantly evolving, so check the official documentation for the most up-to-date information.

## Resources

- [Axum GitHub Repository](https://github.com/tokio-rs/axum)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Tokio Project](https://tokio.rs/)
- [Rust Book](https://doc.rust-lang.org/book/) 

