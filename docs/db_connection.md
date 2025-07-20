# MongoDB Integration Guide

This guide explains how to connect MongoDB to your Rust Axum API project.

## Step 1: Add MongoDB Dependencies

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...
mongodb = "2.8.1"
```

## Step 2: Create Database Module Structure

Create the following folder structure:

```
src/
├── db/
│   ├── mod.rs
│   ├── db.rs          # Base connection
│   └── repositories/  # Database operations
```

## Step 3: Set Up the Connection

Your `db.rs` file manages MongoDB connection:

```rust
use mongodb::{
    bson::doc, 
    options::{ClientOptions, ServerApi, ServerApiVersion}, 
    Client
};

pub async fn get_client() -> mongodb::error::Result<Client> {
  let mut client_options =
    ClientOptions::parse("mongodb+srv://username:password@your-cluster.mongodb.net/?retryWrites=true&w=majority")
      .await?;

  // Set API version
  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);

  // Create and return the client
  Client::with_options(client_options)
}

// Connection test
pub async fn test_connection() -> mongodb::error::Result<()> {
  let client = get_client().await?;
  
  client
    .database("admin")
    .run_command(doc! {"ping": 1}, None)
    .await?;
  println!("Connected to MongoDB successfully!");

  Ok(())
}
```

## Step 4: Create mod.rs to Export Functions

In `src/db/mod.rs`:

```rust
pub mod db;
pub mod repositories;
```

## Step 5: Create User Repository

Create `src/db/repositories/user_repo.rs`:

```rust
use mongodb::{
    bson::{doc, oid::ObjectId}, 
    Collection, Database
};
use crate::model::model::User;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("users");
        Self { collection }
    }

    pub async fn create_user(&self, user: User) -> mongodb::error::Result<User> {
        self.collection.insert_one(user.clone(), None).await?;
        Ok(user)
    }

    pub async fn find_by_id(&self, id: u64) -> mongodb::error::Result<Option<User>> {
        self.collection
            .find_one(doc! { "id": id }, None)
            .await
    }

    pub async fn find_by_email(&self, email: &str) -> mongodb::error::Result<Option<User>> {
        self.collection
            .find_one(doc! { "email": email }, None)
            .await
    }
}
```

## Step 6: Set Up Database Connection in main.rs

Update your `main.rs`:

```rust
mod db;
// Other modules...

#[tokio::main]
async fn main() {
    // Initialize database
    let client = match db::db::get_client().await {
        Ok(client) => {
            println!("Connected to MongoDB successfully!");
            client
        },
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {}", e);
            std::process::exit(1);
        }
    };
    
    let database = client.database("your_database_name");

    // Initialize repositories
    // Pass db handle to routes/services that need database access
    // ...

    // Set up routes and start server
    // ...
}
```

## Step 7: Update Your Model for MongoDB

Make sure your User model is compatible with MongoDB:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}
```

## Step 8: Update Your Handlers to Use Database

Update your handlers to use the database repositories:

```rust
pub async fn create_user(
    State(db): State<Database>,
    Json(payload): Json<User>
) -> impl IntoResponse {
    let repo = UserRepository::new(&db);
    
    match repo.create_user(payload.clone()).await {
        Ok(_) => {
            let response = ApiResponse {
                message: format!("User: {} created", payload.name),
                data: Some(payload),
            };
            (StatusCode::CREATED, Json(response))
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("Failed to create user: {}", e),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}
```

## Important Security Notes

1. **NEVER** hardcode your MongoDB connection string with credentials in your code.
2. Use environment variables or a configuration file to store sensitive information.
3. The connection string in `db.rs` should be replaced with:

```rust
let connection_string = std::env::var("MONGODB_URI")
    .expect("MONGODB_URI must be set");
ClientOptions::parse(&connection_string).await?
```

## Additional Tips

1. Use connection pooling for production applications.
2. Implement proper error handling for database operations.
3. Consider using transactions for operations that modify multiple documents.
4. Create indices for frequently queried fields.
5. Use MongoDB's aggregation framework for complex queries. 