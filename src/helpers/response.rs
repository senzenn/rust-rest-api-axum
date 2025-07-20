use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::Value;
use crate::model::model::{ApiResponse, ErrorResponse};

// Unified response type that can handle both success and error cases
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum UnifiedResponse<T> {
    Success(ApiResponse<T>),
    Error(ErrorResponse),
}

impl<T> IntoResponse for UnifiedResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            UnifiedResponse::Success(response) => {
                let (status, json) = (StatusCode::OK, Json(response));
                (status, json).into_response()
            }
            UnifiedResponse::Error(response) => {
                let (status, json) = (StatusCode::BAD_REQUEST, Json(response));
                (status, json).into_response()
            }
        }
    }
}

// Generic error response that can be converted to any type
pub fn error_response_generic<T>(error: String, message: String) -> UnifiedResponse<T> {
    UnifiedResponse::Error(ErrorResponse { error, message })
}

// Generic not found response that can be converted to any type
pub fn not_found_response_generic<T>(message: String) -> UnifiedResponse<T> {
    UnifiedResponse::Success(ApiResponse {
        message,
        data: None,
    })
}

// Generic SQL error response that can be converted to any type
pub fn sql_error_response_generic<T>(error: anyhow::Error, context: &str) -> UnifiedResponse<T> {
    UnifiedResponse::Error(ErrorResponse {
        error: "Database Error".to_string(),
        message: format!("{}: {}", context, error),
    })
}

pub fn create_response<T>(
    message: String,
    data: Option<T>,
    status_code: StatusCode,
) -> (StatusCode, Json<ApiResponse<T>>) {
    let response = ApiResponse {
        message,
        data,
    };
    (status_code, Json(response))
}

pub fn create_error_response(
    error: String,
    message: String,
    status_code: StatusCode,
) -> (StatusCode, Json<ErrorResponse>) {
    let response = ErrorResponse {
        error,
        message,
    };
    (status_code, Json(response))
}

pub fn handle_mongo_error(error: mongodb::error::Error, error_context: &str) -> (StatusCode, Json<ErrorResponse>) {
    create_error_response(
        "Database Error".to_string(),
        format!("{}: {}", error_context, error),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn handle_sql_error(error: anyhow::Error, error_context: &str) -> (StatusCode, Json<ErrorResponse>) {
    create_error_response(
        "Database Error".to_string(),
        format!("{}: {}", error_context, error),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

// Unified response functions that return the same type
pub fn success_response<T>(message: String, data: T) -> UnifiedResponse<T> {
    UnifiedResponse::Success(ApiResponse {
        message,
        data: Some(data),
    })
}

pub fn error_response(error: String, message: String) -> UnifiedResponse<Value> {
    UnifiedResponse::Error(ErrorResponse { error, message })
}

pub fn not_found_response(message: String) -> UnifiedResponse<Value> {
    UnifiedResponse::Success(ApiResponse {
        message,
        data: None,
    })
}

pub fn sql_error_response(error: anyhow::Error, context: &str) -> UnifiedResponse<Value> {
    UnifiedResponse::Error(ErrorResponse {
        error: "Database Error".to_string(),
        message: format!("{}: {}", context, error),
    })
}

// Legacy functions for backward compatibility
pub fn success_response_legacy(message: String, data: Option<Value>) -> (StatusCode, Json<ApiResponse<Value>>) {
    create_response(message, data, StatusCode::OK)
}

pub fn not_found_response_legacy(message: String) -> (StatusCode, Json<ApiResponse<Value>>) {
    create_response(message, None, StatusCode::NOT_FOUND)
}

pub fn bad_request_response_legacy(message: String) -> (StatusCode, Json<ApiResponse<Value>>) {
    create_response(message, None, StatusCode::BAD_REQUEST)
} 