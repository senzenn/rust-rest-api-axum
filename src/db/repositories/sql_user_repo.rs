use sqlx::{sqlite::SqlitePool, Row};
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::model::model::{User, CreateUserRequest, UpdateUserRequest, UserResponse};
use tracing::{debug, info};

pub struct SqlUserRepository {
    pool: SqlitePool,
}

impl SqlUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        debug!("Creating new SqlUserRepository");
        Self { pool }
    }

    pub async fn create_user(&self, user_data: CreateUserRequest, hashed_password: String) -> Result<User> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        info!("Creating new user with email: {}", user_data.email);
        
        let user = User {
            id,
            name: user_data.name,
            email: user_data.email,
            password: hashed_password,
            created_at: now,
            updated_at: now,
        };

        sqlx::query(
            r#"
            INSERT INTO users (id, name, email, password, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.id.to_string())
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(user.created_at.to_rfc3339())
        .bind(user.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        debug!("User created successfully: id={}", user.id);
        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        debug!("Finding user by id: {}", id);
        
        let row = sqlx::query(
            r#"
            SELECT id, name, email, password, created_at, updated_at
            FROM users WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let user = User {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    name: row.get("name"),
                    email: row.get("email"),
                    password: row.get("password"),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
                };
                debug!("User with id {} found", id);
                Ok(Some(user))
            }
            None => {
                debug!("User with id {} not found", id);
                Ok(None)
            }
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        debug!("Finding user by email: {}", email);
        
        let row = sqlx::query(
            r#"
            SELECT id, name, email, password, created_at, updated_at
            FROM users WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let user = User {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    name: row.get("name"),
                    email: row.get("email"),
                    password: row.get("password"),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
                };
                debug!("User with email {} found", email);
                Ok(Some(user))
            }
            None => {
                debug!("User with email {} not found", email);
                Ok(None)
            }
        }
    }

    pub async fn update_user(&self, id: Uuid, update_data: UpdateUserRequest) -> Result<Option<User>> {
        info!("Updating user with id: {}", id);
        
        // First check if user exists
        let existing_user = self.find_by_id(id).await?;
        if existing_user.is_none() {
            return Ok(None);
        }

        let mut user = existing_user.unwrap();
        let mut updated = false;

        if let Some(name) = update_data.name {
            user.name = name;
            updated = true;
        }

        if let Some(email) = update_data.email {
            user.email = email;
            updated = true;
        }

        if let Some(password) = update_data.password {
            user.password = password;
            updated = true;
        }

        if updated {
            user.updated_at = Utc::now();
            
            sqlx::query(
                r#"
                UPDATE users 
                SET name = ?, email = ?, password = ?, updated_at = ?
                WHERE id = ?
                "#,
            )
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password)
            .bind(user.updated_at.to_rfc3339())
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

            debug!("User with id {} updated successfully", id);
        }

        Ok(Some(user))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<bool> {
        info!("Deleting user with id: {}", id);
        
        let result = sqlx::query(
            r#"
            DELETE FROM users WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            debug!("User with id {} deleted successfully", id);
        } else {
            debug!("No user with id {} found to delete", id);
        }
        
        Ok(deleted)
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>> {
        debug!("Getting all users");
        
        let rows = sqlx::query(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let users: Result<Vec<UserResponse>> = rows
            .into_iter()
            .map(|row| {
                Ok(UserResponse {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    name: row.get("name"),
                    email: row.get("email"),
                    created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
                })
            })
            .collect();

        users
    }
} 