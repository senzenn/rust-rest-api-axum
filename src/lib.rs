pub mod model;
pub mod handlers;
pub mod helpers;
pub mod db;

// Re-exporting  commonly used types for easier access in tests
pub use model::model::{CreateUserRequest, LoginRequest, CreatePostRequest}; 
