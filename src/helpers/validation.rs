use crate::model::model::{User, CreateUserRequest};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
}

/// Helper to validate user data before processing
pub fn validate_user(user: &User) -> Result<(), String> {
    // Email validation
    if !EMAIL_REGEX.is_match(&user.email) {
        return Err("Invalid email format".to_string());
    }
    
    // Password validation
    if user.password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    
    // Name validation
    if user.name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    
    Ok(())
}

/// Helper fn  to validate user registration data
pub fn validate_user_registration(user: &CreateUserRequest) -> Result<(), String> {
    // Email validation
    if !EMAIL_REGEX.is_match(&user.email) {
        return Err("Invalid email format".to_string());
    }
    
    // Password validation
    if user.password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    
    // Name 
    if user.name.trim().is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    
    // Email length 
    if user.email.len() > 255 {
        return Err("Email is too long".to_string());
    }
    
    // Name length
    if user.name.len() > 100 {
        return Err("Name is too long".to_string());
    }
    
    Ok(())
}

///   email format checker
pub fn validate_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

/// check password strength
pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    
    if !has_uppercase || !has_lowercase || !has_digit {
        return Err("Password must contain at least one uppercase letter, one lowercase letter, and one digit".to_string());
    }
    
    Ok(())
} 
