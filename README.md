# Axum Auth API

A complete Rust API with authentication and post management using Axum and SQLite.

## Features

- User authentication (register, login, profile)
- JWT token authentication
- Post CRUD operations
- SQLite database
- Input validation
- CORS support

## Quick Start

1. **Setup**
   ```bash
   git clone <repository-url>
   cd api-rustone
   ```

2. **Environment**
   ```bash
   # Create .env file
   DATABASE_URL=sqlite:./api_rust_one.db
   JWT_SECRET=your-super-secret-jwt-key
   RUST_LOG=info
   ```

3. **Run**
   ```bash
   cargo run
   ```

Server starts at `http://127.0.0.1:8081`

## API Endpoints

### Auth
- `POST /auth/register` - Register user
- `POST /auth/login` - Login user
- `GET /auth/profile` - Get profile (auth required)
- `PUT /auth/profile` - Update profile (auth required)

### Posts
- `GET /posts` - Get all posts
- `GET /posts/{id}` - Get specific post
- `POST /posts` - Create post (auth required)
- `GET /posts/my` - Get user's posts (auth required)
- `PUT /posts/{id}` - Update post (auth required)
- `DELETE /posts/{id}` - Delete post (auth required)

## Testing

### Run Tests
```bash
# All tests
cargo test

# Integration tests only
cargo test --test integration_test

# Unit tests only
cargo test --lib
```

### Manual API Testing
```bash
# Start server
cargo run

# Test endpoints
curl -X POST http://localhost:8081/auth/register \
  -H "Content-Type: application/json" \
  -d '{"name": "Test User", "email": "test@example.com", "password": "TestPass123"}'

curl -X POST http://localhost:8081/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "TestPass123"}'

# Use returned token for protected endpoints
curl -X GET http://localhost:8081/auth/profile \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

## Response Format

Success:
```json
{
  "message": "Success message",
  "data": { ... }
}
```

Error:
```json
{
  "error": "Error type",
  "message": "Error description"
}
```

## Project Structure

```
src/
├── main.rs              # App entry point
├── lib.rs               # Library exports
├── model/               # Data models
├── handlers/            # Route handlers
├── db/                  # Database setup
└── helpers/             # Utilities
```

## Tech Stack

- **Framework**: Axum
- **Database**: SQLite
- **Auth**: JWT + bcrypt
- **Validation**: Custom regex
- **Logging**: Tracing # axum-rest-auth
# rust-rest-api-axum
# axum-rest-auth-axum
# rust-rest-api-axum
# rust-rest-api-axum
