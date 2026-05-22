# Contributing to LLM REST API Server

## Development Setup

### Prerequisites
- Rust 1.70+ (install from https://rustup.rs/)
- Oracle Database connection (local or remote)
- Git

### Local Development

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd my-rust-app
   ```

2. **Setup environment**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. **Run locally**
   ```bash
   cargo run
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

5. **Format code**
   ```bash
   cargo fmt
   ```

6. **Lint code**
   ```bash
   cargo clippy
   ```

### Using Docker Compose (Alternative)

```bash
cp .env.example .env
# Update .env with your database credentials
docker-compose up --build
```

## Code Style & Standards

### Formatting
- Use `cargo fmt` before committing
- Line length: 100 characters (configurable)
- Indentation: 4 spaces (Rust default)

### Linting
- Run `cargo clippy` and fix all warnings
- Follow Rust API guidelines (https://rust-lang.github.io/api-guidelines/)

### Documentation
- Document public APIs with doc comments
- Include examples for complex functions
- Update README.md if adding new features

```rust
/// Fetches all LLM models from the database.
///
/// # Arguments
/// * `settings` - Application settings containing database connection info
///
/// # Returns
/// Result containing vector of LlmInfo or AppError
///
/// # Errors
/// Returns AppError if database connection fails or query fails
pub fn get_llm_info(settings: &Settings) -> Result<Vec<LlmInfo>, AppError> {
    // Implementation
}
```

## Project Structure

```
src/
├── main.rs              # Application entry point
├── api/                 # HTTP handlers and routes
├── config/              # Configuration management
├── db/                  # Database layer
├── domain/              # Domain models
├── error/               # Error types and handling
├── repository/          # Data access layer
├── service/             # Business logic
└── utils/               # Utilities and helpers
```

## Adding New Features

### Adding a New API Endpoint

1. **Create a handler** in `src/api/handlers/<feature>_handler.rs`:
   ```rust
   use axum::{extract::State, Json};
   use crate::config::Settings;
   use crate::error::AppError;
   
   pub async fn my_handler(
       State(settings): State<Settings>,
   ) -> Result<Json<MyResponse>, AppError> {
       // Implementation
       Ok(Json(MyResponse { /* ... */ }))
   }
   ```

2. **Update handler module** (`src/api/handlers/mod.rs`):
   ```rust
   pub mod feature_handler;
   ```

3. **Register route** in `src/main.rs`:
   ```rust
   let app = Router::new()
       .route("/endpoint", post(api::handlers::feature_handler::my_handler))
       .with_state(settings.clone());
   ```

### Adding a New Service

1. **Create service file** (`src/service/<feature>_service.rs`)
2. **Add to service module** (`src/service/mod.rs`)
3. **Update handler** to use the new service
4. **Add tests** for the service

### Adding Database Operations

1. **Create or update repository** (`src/repository/<feature>_repo.rs`)
2. **Add to repository module** (`src/repository/mod.rs`)
3. **Create service** using the repository
4. **Add handler** using the service

## Error Handling

Always use the `AppError` enum for errors:

```rust
use crate::error::AppError;

pub async fn my_handler() -> Result<Json<Response>, AppError> {
    // Validation errors
    if invalid {
        return Err(AppError::ValidationError("Field is required".to_string()));
    }
    
    // Database errors (automatically converted)
    let data = fetch_from_db()
        .map_err(AppError::from)?;
    
    // Not found errors
    if data.is_empty() {
        return Err(AppError::NotFound("Resource not found".to_string()));
    }
    
    Ok(Json(response))
}
```

## Testing

### Write Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = setup_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_output);
    }
}
```

### Write Integration Tests

Create tests in `tests/` directory:

```bash
tests/
├── common/
│   └── mod.rs
└── api_tests.rs
```

### Run Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Only doc tests
cargo test --doc
```

## Logging

Use the `tracing` macros for logging:

```rust
use tracing::{info, warn, error, debug};

#[tokio::main]
async fn main() {
    info!("Application starting");
    warn!("This is a warning");
    error!("An error occurred");
    debug!("Debug information");
}
```

Configure log level in `.env`:
```env
LOG_LEVEL=debug  # debug, info, warn, error, trace
```

## Performance Considerations

1. **Connection Pooling**: Consider adding `r2d2-oracle` for production
2. **Caching**: Cache frequently accessed data
3. **Query Optimization**: Profile queries for performance
4. **Async I/O**: Use async/await throughout
5. **Memory**: Monitor for memory leaks in long-running processes

## Security Guidelines

1. **Never hardcode credentials** - use environment variables
2. **Validate all inputs** - use the `validator` crate
3. **Sanitize errors** - don't expose internal details
4. **Use HTTPS** - in production
5. **Implement authentication** - for sensitive endpoints
6. **SQL Injection prevention** - use parameterized queries (Oracle)

## Deployment

### Building for Production

```bash
cargo build --release
```

### Building Docker Image

```bash
docker build -t llm-api:latest .
docker run -p 3000:3000 --env-file .env llm-api:latest
```

### Health Check

```bash
curl http://localhost:3000/health
```

## Debugging

### Enable Debug Logging

```env
LOG_LEVEL=debug
RUST_BACKTRACE=1
```

### Common Issues

1. **"DATABASE_URL not set"**
   - Verify .env file exists and is loaded
   - Run `cargo run` from project root

2. **"Connection refused"**
   - Check database connection string
   - Verify database is running

3. **"Port already in use"**
   - Change SERVER_PORT in .env
   - Kill process using the port

## Git Workflow

1. Create feature branch: `git checkout -b feature/my-feature`
2. Make changes and commit: `git commit -m "Add my feature"`
3. Push branch: `git push origin feature/my-feature`
4. Create pull request
5. Address review comments
6. Merge when approved

## Commit Message Convention

```
feat: add new feature
fix: fix bug in feature
docs: update documentation
test: add tests
refactor: refactor code
chore: update dependencies
```

## Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Documentation](https://tokio.rs/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Tracing Documentation](https://docs.rs/tracing/)
- [Oracle Rust Driver](https://docs.rs/oracle/)

## Getting Help

- Check existing documentation in README.md
- Review examples in the codebase
- Check GitHub issues for similar problems
- Ask in discussions or create an issue

## Reporting Bugs

When reporting bugs, include:
1. Description of the bug
2. Steps to reproduce
3. Expected vs actual behavior
4. Environment (OS, Rust version, database)
5. Relevant logs (with LOG_LEVEL=debug)
