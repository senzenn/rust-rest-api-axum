use sqlx::{sqlite::SqlitePool, Row};
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::model::model::{Post, CreatePostRequest, UpdatePostRequest, PostResponse, UserResponse};
use tracing::{debug, info};

pub struct SqlPostRepository {
    pool: SqlitePool,
}

impl SqlPostRepository {
    pub fn new(pool: SqlitePool) -> Self {
        debug!("Creating new SqlPostRepository");
        Self { pool }
    }

    pub async fn create_post(&self, post_data: CreatePostRequest, author_id: Uuid) -> Result<Post> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        info!("Creating new post with title: {}", post_data.title);
        
        let post = Post {
            id,
            title: post_data.title,
            content: post_data.content,
            author_id,
            created_at: now,
            updated_at: now,
        };

        sqlx::query(
            r#"
            INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(post.id.to_string())
        .bind(&post.title)
        .bind(&post.content)
        .bind(post.author_id.to_string())
        .bind(post.created_at.to_rfc3339())
        .bind(post.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        debug!("Post created successfully: id={}", post.id);
        Ok(post)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>> {
        debug!("Finding post by id: {}", id);
        
        let row = sqlx::query(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let post = Post {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    title: row.get("title"),
                    content: row.get("content"),
                    author_id: Uuid::parse_str(&row.get::<String, _>("author_id"))?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
                };
                debug!("Post with id {} found", id);
                Ok(Some(post))
            }
            None => {
                debug!("Post with id {} not found", id);
                Ok(None)
            }
        }
    }

    pub async fn find_by_id_with_author(&self, id: Uuid) -> Result<Option<PostResponse>> {
        debug!("Finding post by id with author: {}", id);
        
        let row = sqlx::query(
            r#"
            SELECT 
                p.id, p.title, p.content, p.author_id, p.created_at, p.updated_at,
                u.name as author_name, u.email as author_email, u.created_at as author_created_at, u.updated_at as author_updated_at
            FROM posts p
            JOIN users u ON p.author_id = u.id
            WHERE p.id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let author = UserResponse {
                    id: Uuid::parse_str(&row.get::<String, _>("author_id"))?,
                    name: row.get("author_name"),
                    email: row.get("author_email"),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("author_created_at"))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("author_updated_at"))?.with_timezone(&Utc),
                };

                let post_response = PostResponse {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    title: row.get("title"),
                    content: row.get("content"),
                    author,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
                };
                
                debug!("Post with id {} found", id);
                Ok(Some(post_response))
            }
            None => {
                debug!("Post with id {} not found", id);
                Ok(None)
            }
        }
    }

    pub async fn find_by_author(&self, author_id: Uuid) -> Result<Vec<Post>> {
        debug!("Finding posts by author: {}", author_id);
        
        let rows = sqlx::query(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts WHERE author_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(author_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        let posts: Result<Vec<Post>> = rows
            .into_iter()
            .map(|row| {
                Ok(Post {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    title: row.get("title"),
                    content: row.get("content"),
                    author_id: Uuid::parse_str(&row.get::<String, _>("author_id"))?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
                })
            })
            .collect();

        posts
    }

    pub async fn update_post(&self, id: Uuid, author_id: Uuid, update_data: UpdatePostRequest) -> Result<Option<Post>> {
        info!("Updating post with id: {}", id);
        
        // First check if post exists and belongs to the author
        let existing_post = self.find_by_id(id).await?;
        if existing_post.is_none() {
            return Ok(None);
        }

        let post = existing_post.unwrap();
        if post.author_id != author_id {
            return Ok(None); // Not authorized
        }

        let mut updated_post = post;
        let mut updated = false;

        if let Some(title) = update_data.title {
            updated_post.title = title;
            updated = true;
        }

        if let Some(content) = update_data.content {
            updated_post.content = content;
            updated = true;
        }

        if updated {
            updated_post.updated_at = Utc::now();
            
            sqlx::query(
                r#"
                UPDATE posts 
                SET title = ?, content = ?, updated_at = ?
                WHERE id = ?
                "#,
            )
            .bind(&updated_post.title)
            .bind(&updated_post.content)
            .bind(updated_post.updated_at.to_rfc3339())
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

            debug!("Post with id {} updated successfully", id);
        }

        Ok(Some(updated_post))
    }

    pub async fn delete_post(&self, id: Uuid, author_id: Uuid) -> Result<bool> {
        info!("Deleting post with id: {}", id);
        
        // First check if post exists and belongs to the author
        let existing_post = self.find_by_id(id).await?;
        if existing_post.is_none() {
            return Ok(false);
        }

        let post = existing_post.unwrap();
        if post.author_id != author_id {
            return Ok(false); // Not authorized
        }
        
        let result = sqlx::query(
            r#"
            DELETE FROM posts WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            debug!("Post with id {} deleted successfully", id);
        } else {
            debug!("No post with id {} found to delete", id);
        }
        
        Ok(deleted)
    }

    pub async fn get_all_posts(&self) -> Result<Vec<PostResponse>> {
        debug!("Getting all posts");
        
        let rows = sqlx::query(
            r#"
            SELECT 
                p.id, p.title, p.content, p.author_id, p.created_at, p.updated_at,
                u.name as author_name, u.email as author_email, u.created_at as author_created_at, u.updated_at as author_updated_at
            FROM posts p
            JOIN users u ON p.author_id = u.id
            ORDER BY p.created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let posts: Result<Vec<PostResponse>> = rows
            .into_iter()
            .map(|row| {
                let author = UserResponse {
                    id: Uuid::parse_str(&row.get::<String, _>("author_id"))?,
                    name: row.get("author_name"),
                    email: row.get("author_email"),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("author_created_at"))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("author_updated_at"))?.with_timezone(&Utc),
                };

                Ok(PostResponse {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    title: row.get("title"),
                    content: row.get("content"),
                    author,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
                })
            })
            .collect();

        posts
    }
} 