use axum::{
    extract::{State, Extension},
    Json,
};
use std::sync::Arc;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::model::model::{
    CreateUserRequest, LoginRequest, LoginResponse, UpdateUserRequest, UserResponse
};
use crate::db::repositories::sql_user_repo::SqlUserRepository;
use crate::helpers::auth::AuthHelper;
use crate::helpers::validation::validate_user_registration;
use crate::helpers::response::{UnifiedResponse, success_response, error_response_generic, not_found_response_generic, sql_error_response_generic};
use tracing::{info, error};

pub async fn register_user(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<CreateUserRequest>
) -> UnifiedResponse<UserResponse> {
    info!("Handler: Registering new user: {}", payload.email);
    
    // Validate input
    if let Err(validation_error) = validate_user_registration(&payload) {
        return error_response_generic("Validation Error".to_string(), validation_error);
    }

    let repo = SqlUserRepository::new((*pool).clone());
    
    // Check existing
    match repo.find_by_email(&payload.email).await {
        Ok(Some(_)) => {
            return error_response_generic("Conflict".to_string(), "User with this email already exists".to_string());
        },
        Ok(None) => {},
        Err(e) => {
            error!("Handler: Failed to check existing user: {}", e);
            return sql_error_response_generic(e, "Failed to check existing user");
        }
    }

    // Hash password
    let hashed_password = match AuthHelper::hash_password(&payload.password) {
        Ok(hashed) => hashed,
        Err(e) => {
            error!("Handler: Failed to hash password: {}", e);
            return error_response_generic("Internal Error".to_string(), "Failed to process password".to_string());
        }
    };

    // Create user
    match repo.create_user(payload.clone(), hashed_password).await {
        Ok(user) => {
            let user_name = user.name.clone();
            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                created_at: user.created_at,
                updated_at: user.updated_at,
            };
            
            success_response(
                format!("User: {} registered successfully", user_name),
                user_response
            )
        },
        Err(e) => {
            error!("Handler: Failed to create user: {}", e);
            sql_error_response_generic(e, "Failed to create user")
        }
    }
}

pub async fn login_user(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<LoginRequest>
) -> UnifiedResponse<LoginResponse> {
    info!("Handler: User login attempt: {}", payload.email);

    let repo = SqlUserRepository::new((*pool).clone());
    
    // Find user
    let user = match repo.find_by_email(&payload.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return error_response_generic("Unauthorized".to_string(), "Invalid email or password".to_string());
        },
        Err(e) => {
            error!("Handler: Failed to find user: {}", e);
            return sql_error_response_generic(e, "Failed to authenticate user");
        }
    };

    // Verify password
    match AuthHelper::verify_password(&payload.password, &user.password) {
        Ok(true) => {
            // Generate token
            let token = match AuthHelper::generate_token(user.id) {
                Ok(token) => token,
                Err(e) => {
                    error!("Handler: Failed to generate token: {}", e);
                    return error_response_generic("Internal Error".to_string(), "Failed to generate authentication token".to_string());
                }
            };

            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                created_at: user.created_at,
                updated_at: user.updated_at,
            };

            let login_response = LoginResponse {
                token,
                user: user_response,
            };

            success_response("Login successful".to_string(), login_response)
        },
        Ok(false) => {
            error_response_generic("Unauthorized".to_string(), "Invalid email or password".to_string())
        },
        Err(e) => {
            error!("Handler: Failed to verify password: {}", e);
            error_response_generic("Internal Error".to_string(), "Failed to verify password".to_string())
        }
    }
}

pub async fn get_profile(
    State(pool): State<Arc<SqlitePool>>,
    Extension(user_id): Extension<Uuid>
) -> UnifiedResponse<UserResponse> {
    info!("Handler: Getting profile for user: {}", user_id);

    let repo = SqlUserRepository::new((*pool).clone());
    
    match repo.find_by_id(user_id).await {
        Ok(Some(user)) => {
            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                created_at: user.created_at,
                updated_at: user.updated_at,
            };
            
            success_response("Profile retrieved successfully".to_string(), user_response)
        },
        Ok(None) => {
            not_found_response_generic("User not found".to_string())
        },
        Err(e) => {
            error!("Handler: Failed to get user profile: {}", e);
            sql_error_response_generic(e, "Failed to get user profile")
        }
    }
}

pub async fn update_profile(
    State(pool): State<Arc<SqlitePool>>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<UpdateUserRequest>
) -> UnifiedResponse<UserResponse> {
    info!("Handler: Updating profile for user: {}", user_id);

    let repo = SqlUserRepository::new((*pool).clone());
    
    // Hash password
    let mut update_data = payload;
    if let Some(password) = &update_data.password {
        match AuthHelper::hash_password(password) {
            Ok(hashed) => update_data.password = Some(hashed),
            Err(e) => {
                error!("Handler: Failed to hash password: {}", e);
                return error_response_generic("Internal Error".to_string(), "Failed to process password".to_string());
            }
        }
    }
    
    match repo.update_user(user_id, update_data).await {
        Ok(Some(user)) => {
            let user_response = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
                created_at: user.created_at,
                updated_at: user.updated_at,
            };
            
            success_response("Profile updated successfully".to_string(), user_response)
        },
        Ok(None) => {
            not_found_response_generic("User not found".to_string())
        },
        Err(e) => {
            error!("Handler: Failed to update user profile: {}", e);
            sql_error_response_generic(e, "Failed to update user profile")
        }
    }
} 