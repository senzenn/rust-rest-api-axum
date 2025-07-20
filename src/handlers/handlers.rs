// Legacy MongoDB handlers - commented out since we're using SQLite now
/*
use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use mongodb::{Client, Database};
use crate::model::model::{User, UpdateUser};
use crate::db::repositories::user_repo::UserRepository;
use crate::helpers::response::{create_response, handle_mongo_error};
use tracing::{info, error};

pub async fn root ()-> & 'static str{
    "Hello, World!"
}

pub async fn get_users(
    State(db): State<Arc<Database>>
) -> impl IntoResponse {
    info!("Handler: Getting all users");
    
    let repo = UserRepository::new(&db);
    
    match repo.get_all_users().await {
        Ok(users) => {
            let response: ApiResponse<User> = ApiResponse {
                message: format!("Retrieved {} users", users.len()),
                data: Some(users),
            };
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            error!("Handler: Failed to get users: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn create_user(
    State(db): State<Arc<Database>>,
    Json(payload): Json<User>
) -> impl IntoResponse {
    info!("Handler: Creating new user: {}", payload.email);
    
    // Validate the user data
    if let Err(validation_error) = validate_user(&payload) {
        return create_response(validation_error, None, StatusCode::BAD_REQUEST);
    }

    let repo = UserRepository::new(&db);
    
    match repo.create_user(payload).await {
        Ok(user) => {
            let response: ApiResponse<User> = ApiResponse {
                message: format!("User: {} created successfully", user.name),
                data: Some(user),
            };
            (StatusCode::CREATED, Json(response))
        },
        Err(e) => {
            error!("Handler: Failed to create user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn update_user(
    State(db): State<Arc<Database>>,
    Json(payload): Json<User>
) -> impl IntoResponse {
    info!("Handler: Updating user: {}", payload.id);
    
    let repo = UserRepository::new(&db);
    
    match repo.update_user(payload).await {
        Ok(user) => {
            let response: ApiResponse<User> = ApiResponse {
                message: format!("User: {} updated successfully", user.name),
                data: Some(user),
            };
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            error!("Handler: Failed to update user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_user_by_id(
    State(db): State<Arc<Database>>,
    Path(id): Path<u64>
) -> impl IntoResponse {
    info!("Handler: Getting user by ID: {}", id);
    
    let repo = UserRepository::new(&db);
    
    match repo.find_by_id(id).await {
        Ok(Some(user)) => {
            let response: ApiResponse<User> = ApiResponse {
                message: "User retrieved successfully".to_string(),
                data: Some(user),
            };
            (StatusCode::OK, Json(response))
        },
        Ok(None) => {
            let response: ApiResponse<User> = ApiResponse {
                message: "User not found".to_string(),
                data: None,
            };
            (StatusCode::NOT_FOUND, Json(response))
        },
        Err(e) => {
            error!("Handler: Failed to get user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_user_by_email(
    State(db): State<Arc<Database>>,
    Path(email): Path<String>
) -> impl IntoResponse {
    info!("Handler: Getting user by email: {}", email);
    
    let repo = UserRepository::new(&db);
    
    match repo.find_by_email(&email).await {
        Ok(Some(user)) => {
            let response: ApiResponse<User> = ApiResponse {
                message: "User retrieved successfully".to_string(),
                data: Some(user),
            };
            (StatusCode::OK, Json(response))
        },
        Ok(None) => {
            let response: ApiResponse<User> = ApiResponse {
                message: "User not found".to_string(),
                data: None,
            };
            (StatusCode::NOT_FOUND, Json(response))
        },
        Err(e) => {
            error!("Handler: Failed to get user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn delete_user(
    State(db): State<Arc<Database>>,
    Path(id): Path<u64>
) -> impl IntoResponse {
    info!("Handler: Deleting user: {}", id);
    
    let repo = UserRepository::new(&db);
    
    match repo.delete_user(id).await {
        Ok(true) => {
            let response: ApiResponse<User> = ApiResponse {
                message: "User deleted successfully".to_string(),
                data: None,
            };
            (StatusCode::OK, Json(response))
        },
        Ok(false) => {
            let response: ApiResponse<User> = ApiResponse {
                message: "User not found".to_string(),
                data: None,
            };
            (StatusCode::NOT_FOUND, Json(response))
        },
        Err(e) => {
            error!("Handler: Failed to delete user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}
*/

pub async fn root() -> &'static str {
    "Hello, World!"
}
