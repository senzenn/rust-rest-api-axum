use axum::{
    extract::{State, Extension, Path},
    Json,
};
use std::sync::Arc;
use sqlx::SqlitePool;
use uuid::Uuid;
use serde_json::Value;
use crate::model::model::{
    CreatePostRequest, UpdatePostRequest, PostResponse
};
use crate::db::repositories::sql_post_repo::SqlPostRepository;
use crate::helpers::response::{UnifiedResponse, success_response, error_response_generic, not_found_response_generic, sql_error_response_generic};
use tracing::{info, error};

pub async fn create_post(
    State(pool): State<Arc<SqlitePool>>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreatePostRequest>
) -> UnifiedResponse<PostResponse> {
    info!("Handler: Creating new post for user: {}", user_id);
    
    // Validate input
    if payload.title.trim().is_empty() {
        return error_response_generic("Bad Request".to_string(), "Post title cannot be empty".to_string());
    }

    if payload.content.trim().is_empty() {
        return error_response_generic("Bad Request".to_string(), "Post content cannot be empty".to_string());
    }

    let repo = SqlPostRepository::new((*pool).clone());
    
    match repo.create_post(payload, user_id).await {
        Ok(post) => {
            // Get with author
            match repo.find_by_id_with_author(post.id).await {
                Ok(Some(post_response)) => {
                    success_response(
                        format!("Post '{}' created successfully", post.title),
                        post_response
                    )
                },
                Ok(None) => {
                    error_response_generic("Internal Error".to_string(), "Post created but failed to retrieve with author info".to_string())
                },
                Err(e) => {
                    error!("Handler: Failed to get post with author: {}", e);
                    sql_error_response_generic(e, "Failed to get post with author")
                }
            }
        },
        Err(e) => {
            error!("Handler: Failed to create post: {}", e);
            sql_error_response_generic(e, "Failed to create post")
        }
    }
}

pub async fn get_post(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<Uuid>
) -> UnifiedResponse<PostResponse> {
    info!("Handler: Getting post: {}", id);

    let repo = SqlPostRepository::new((*pool).clone());
    
    match repo.find_by_id_with_author(id).await {
        Ok(Some(post)) => {
            success_response("Post retrieved successfully".to_string(), post)
        },
        Ok(None) => {
            not_found_response_generic("Post not found".to_string())
        },
        Err(e) => {
            error!("Handler: Failed to get post: {}", e);
            sql_error_response_generic(e, "Failed to get post")
        }
    }
}

pub async fn get_user_posts(
    State(pool): State<Arc<SqlitePool>>,
    Extension(user_id): Extension<Uuid>
) -> UnifiedResponse<Vec<crate::model::model::Post>> {
    info!("Handler: Getting posts for user: {}", user_id);

    let repo = SqlPostRepository::new((*pool).clone());
    
    match repo.find_by_author(user_id).await {
        Ok(posts) => {
            success_response(
                format!("Retrieved {} posts", posts.len()),
                posts
            )
        },
        Err(e) => {
            error!("Handler: Failed to get user posts: {}", e);
            sql_error_response_generic(e, "Failed to get user posts")
        }
    }
}

pub async fn get_all_posts(
    State(pool): State<Arc<SqlitePool>>
) -> UnifiedResponse<Vec<PostResponse>> {
    info!("Handler: Getting all posts");

    let repo = SqlPostRepository::new((*pool).clone());
    
    match repo.get_all_posts().await {
        Ok(posts) => {
            success_response(
                format!("Retrieved {} posts", posts.len()),
                posts
            )
        },
        Err(e) => {
            error!("Handler: Failed to get all posts: {}", e);
            sql_error_response_generic(e, "Failed to get all posts")
        }
    }
}

pub async fn update_post(
    State(pool): State<Arc<SqlitePool>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePostRequest>
) -> UnifiedResponse<PostResponse> {
    info!("Handler: Updating post: {} for user: {}", id, user_id);

    let repo = SqlPostRepository::new((*pool).clone());
    
    match repo.update_post(id, user_id, payload).await {
        Ok(Some(post)) => {
            // Get with author
            match repo.find_by_id_with_author(post.id).await {
                Ok(Some(post_response)) => {
                    success_response(
                        format!("Post '{}' updated successfully", post.title),
                        post_response
                    )
                },
                Ok(None) => {
                    error_response_generic("Internal Error".to_string(), "Post updated but failed to retrieve with author info".to_string())
                },
                Err(e) => {
                    error!("Handler: Failed to get updated post with author: {}", e);
                    sql_error_response_generic(e, "Failed to get updated post with author")
                }
            }
        },
        Ok(None) => {
            not_found_response_generic("Post not found or you don't have permission to update it".to_string())
        },
        Err(e) => {
            error!("Handler: Failed to update post: {}", e);
            sql_error_response_generic(e, "Failed to update post")
        }
    }
}

pub async fn delete_post(
    State(pool): State<Arc<SqlitePool>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>
) -> UnifiedResponse<Value> {
    info!("Handler: Deleting post: {} for user: {}", id, user_id);

    let repo = SqlPostRepository::new((*pool).clone());
    
    match repo.delete_post(id, user_id).await {
        Ok(true) => {
            success_response("Post deleted successfully".to_string(), Value::Null)
        },
        Ok(false) => {
            not_found_response_generic("Post not found or you don't have permission to delete it".to_string())
        },
        Err(e) => {
            error!("Handler: Failed to delete post: {}", e);
            sql_error_response_generic(e, "Failed to delete post")
        }
    }
} 