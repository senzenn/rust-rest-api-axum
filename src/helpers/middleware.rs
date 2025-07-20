use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};

use crate::helpers::auth::AuthHelper;
use crate::model::model::ErrorResponse;
use tracing::{error, info};

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        });

    let token = match auth_header {
        Some(token) => token,
        None => {
            error!("No authorization header found");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Unauthorized".to_string(),
                    message: "No authorization header found".to_string(),
                }),
            ));
        }
    };

    let user_id = match AuthHelper::extract_user_id_from_token(&token) {
        Ok(user_id) => user_id,
        Err(e) => {
            error!("Invalid token: {}", e);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Unauthorized".to_string(),
                    message: "Invalid token".to_string(),
                }),
            ));
        }
    };

    info!("Authenticated user: {}", user_id);
    
    // Add user_id to request extensions
    request.extensions_mut().insert(user_id);
    
    Ok(next.run(request).await)
}

pub async fn optional_auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        });

    if let Some(token) = auth_header {
        if let Ok(user_id) = AuthHelper::extract_user_id_from_token(&token) {
            info!("Optional authentication successful for user: {}", user_id);
            request.extensions_mut().insert(Some(user_id));
        } else {
            request.extensions_mut().insert(None::<uuid::Uuid>);
        }
    } else {
        request.extensions_mut().insert(None::<uuid::Uuid>);
    }
    
    Ok(next.run(request).await)
} 