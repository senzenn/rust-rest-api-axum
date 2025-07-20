use sqlx::sqlite::SqlitePool;
use anyhow::Result;
use tracing::info;

pub struct SqlDatabase {
    pool: SqlitePool,
}

impl SqlDatabase {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        info!("Connected to SQLite database");
        
        // Initialize database tables
        Self::init_database(&pool).await?;
        
        Ok(Self { pool })
    }

    async fn init_database(pool: &SqlitePool) -> Result<()> {
        // Create users table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create posts table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS posts (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                author_id TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL,
                FOREIGN KEY (author_id) REFERENCES users (id)
            )
            "#,
        )
        .execute(pool)
        .await?;

        info!("Database tables initialized successfully");
        Ok(())
    }

    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}

pub async fn get_sql_client() -> Result<SqlDatabase> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./api_rust_one.db".to_string());
    
    SqlDatabase::new(&database_url).await
} 