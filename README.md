# LLM REST API Server

A professional Rust REST API server for managing LLM models, built with Axum and featuring proper error handling, logging, and configuration management.

## Features

- **Axum Web Framework**: Modern async web server
- **Proper Error Handling**: Structured error responses with appropriate HTTP status codes
- **Structured Logging**: Request tracing and application logging with `tracing` and `tracing-subscriber`
- **Configuration Management**: Environment-based configuration with sensible defaults
- **Health Checks**: Built-in health check endpoint for monitoring
- **Oracle Database**: Integration with Oracle database for model metadata
- **Production Ready**: Proper response formats, error handling, and middleware

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── api/                    # HTTP routes and handlers
│   ├── handlers/           # Request handlers
│   │   ├── health_handler.rs
│   │   └── model_handler.rs
│   └── routes.rs          # Route definitions
├── config/                 # Configuration management
│   └── settings.rs        # Settings from environment
├── db/                     # Database layer
│   └── connection.rs      # Database connections
├── domain/                 # Domain models
│   └── models/            # Data models
├── error/                  # Error handling
│   └── app_error.rs       # Application errors
├── repository/             # Data access layer
│   └── model_repo.rs      # Model queries
├── service/                # Business logic layer
│   └── model_service.rs   # Model operations
└── utils/                  # Utility functions
    └── helpers.rs         # Logging setup, helpers
```

## Getting Started

### Prerequisites

- Rust 1.70+ (edition 2021)
- Oracle Database connection
- Cargo

### Installation

1. Clone the repository
2. Copy `.env.example` to `.env` and update with your configuration:

```bash
cp .env.example .env
```

3. Update `.env` with your database credentials:

```env
DATABASE_URL=your_oracle_connection_string
DB_USER=your_database_user
DB_PASSWORD=your_database_password
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
LOG_LEVEL=info
ENVIRONMENT=development
```

### Building

```bash
cargo build --release
```

### Running

```bash
cargo run
```

The server will start and listen on the configured address (default: `http://127.0.0.1:3000`).

## API Endpoints

### Health Check

```
GET /health
```

Returns server health status.

**Response:**
```json
{
  "status": "healthy",
  "message": "Server is running"
}
```

### List Models

```
GET /models
```

Retrieves all available LLM models from the database.

**Response:**
```json
{
  "models": [
    {
      "id": 1,
      "model_name": "GPT-4",
      "version": "1.0",
      "provider": "OpenAI"
    }
  ]
}
```

**Error Response (404):**
```json
{
  "error": "NOT_FOUND",
  "message": "No models found",
  "details": "The requested resource does not exist"
}
```

## Configuration

Environment variables:

- `SERVER_HOST`: Server binding address (default: `127.0.0.1`)
- `SERVER_PORT`: Server port (default: `3000`)
- `DATABASE_URL`: Oracle connection string
- `DB_USER`: Database username
- `DB_PASSWORD`: Database password
- `LOG_LEVEL`: Logging level - `error`, `warn`, `info`, `debug`, `trace` (default: `info`)
- `ENVIRONMENT`: Deployment environment - `development`, `staging`, `production` (default: `development`)

## Logging

The application uses structured logging with `tracing`. Logs include:
- Request entry and exit
- Timing information
- Error details
- Application lifecycle events

### Log Format

```
TIMESTAMP [LEVEL] [TARGET] MESSAGE
```

Example:
```
2024-05-22T10:30:45.123Z [INFO] my_rust_app: === LLM REST API Server Starting ===
2024-05-22T10:30:45.124Z [INFO] my_rust_app: Environment: development
```

## Error Handling

The application provides structured error responses:

- **400 Bad Request**: Validation errors
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Server errors (database, config, etc.)

All errors include:
- Error type
- Error message
- Optional details for debugging

## Development

### Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `serde` - Serialization
- `oracle` - Database driver
- `tracing` - Structured logging
- `thiserror` - Error handling

### Adding New Endpoints

1. Create a handler in `src/api/handlers/`
2. Add the route in `src/api/routes.rs`
3. Import and register in `src/main.rs`

## Database Schema

The application expects the following table for models:

```sql
CREATE TABLE llm_metadata (
    id NUMBER PRIMARY KEY,
    model_name VARCHAR2(255),
    version VARCHAR2(50),
    provider VARCHAR2(255)
);
```

## Performance Considerations

- Connection pooling for database connections (recommended for production)
- Structured logging with async writers
- Request tracing middleware for performance monitoring

## Security

- Environment variables for sensitive data (never hardcode credentials)
- Proper error messages without exposing internal details
- Request validation on API endpoints

## License

MIT

## Support

For issues and questions, please create an issue in the repository.
