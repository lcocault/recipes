# Recipes API

A REST API service for managing ingredients in a recipes and cost calculator application.

## Features

- **Ingredient Management**: CRUD operations for ingredients
- **In-memory Storage**: Thread-safe in-memory repository with clean abstraction
- **OpenAPI Compliant**: Implements the `/ingredients` endpoints defined in the OpenAPI specification
- **Fully Tested**: Comprehensive unit and integration tests

## Technology Stack

- **Language**: Rust
- **Framework**: Axum (web framework)
- **Runtime**: Tokio (async runtime)
- **Serialization**: Serde + serde_json
- **Testing**: Tokio test, Tower ServiceExt

## Architecture

The application follows a modular, layered architecture:

```
src/
├── main.rs              # Application entry point and server setup
├── lib.rs               # Library root for public exports
├── handlers/            # HTTP request handlers
│   ├── mod.rs
│   └── ingredients.rs   # Ingredient endpoint handlers
├── models/              # Data models
│   ├── mod.rs
│   └── ingredient.rs    # Ingredient model and DTOs
└── repository/          # Data persistence layer
    ├── mod.rs
    └── ingredient_repository.rs  # Repository trait and in-memory implementation
```

### Design Principles

- **Separation of Concerns**: Clear separation between HTTP handlers, business logic, and data access
- **Dependency Injection**: Repository is injected into handlers via Axum state
- **Abstraction**: Repository trait allows easy swapping of storage implementations
- **Thread Safety**: In-memory storage uses `Arc<RwLock<T>>` for safe concurrent access

## API Endpoints

All endpoints are defined in `doc/openapi.yaml`.

### GET /ingredients
List all ingredients.

**Response**: 200 OK
```json
[
  {
    "id": "uuid",
    "name": "Flour",
    "defaultUnit": "kg",
    "purchaseUnit": "kg",
    "price": 2.5,
    "currency": "EUR",
    "density": 0.8,
    "notes": "All-purpose flour"
  }
]
```

### POST /ingredients
Create a new ingredient.

**Request Body**:
```json
{
  "name": "Flour",
  "defaultUnit": "kg",
  "purchaseUnit": "kg",
  "price": 2.5,
  "currency": "EUR",
  "density": 0.8,
  "notes": "All-purpose flour"
}
```

**Response**: 201 Created
```json
{
  "id": "generated-uuid",
  "name": "Flour",
  "defaultUnit": "kg",
  "purchaseUnit": "kg",
  "price": 2.5,
  "currency": "EUR",
  "density": 0.8,
  "notes": "All-purpose flour"
}
```

**Validation**:
- Name must not be empty
- Price must not be negative

### PUT /ingredients/{id}
Update an existing ingredient.

**Request Body**: Same as POST

**Response**: 200 OK (updated ingredient) or 404 Not Found

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/lcocault/recipes.git
cd recipes
```

2. Build the project:
```bash
cargo build
```

### Running the Server

Start the server on port 8080:

```bash
cargo run
```

The server will be available at `http://localhost:8080`.

### Testing

Run all tests (unit + integration):

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

Run specific test:

```bash
cargo test test_create_ingredient
```

### Development

Format code:
```bash
cargo fmt
```

Run linter:
```bash
cargo clippy
```

Build for release:
```bash
cargo build --release
```

## Example Usage

### Create an ingredient

```bash
curl -X POST http://localhost:8080/ingredients \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Flour",
    "defaultUnit": "kg",
    "purchaseUnit": "kg",
    "price": 2.5,
    "currency": "EUR",
    "density": 0.8,
    "notes": "All-purpose flour"
  }'
```

### List all ingredients

```bash
curl http://localhost:8080/ingredients
```

### Update an ingredient

```bash
curl -X PUT http://localhost:8080/ingredients/{id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Whole Wheat Flour",
    "defaultUnit": "kg",
    "purchaseUnit": "kg",
    "price": 3.0,
    "currency": "EUR",
    "density": 0.75
  }'
```

## Project Structure

- `src/` - Source code
- `tests/` - Integration tests
- `doc/` - Documentation including OpenAPI spec
- `Cargo.toml` - Project dependencies and metadata
- `Cargo.lock` - Locked dependency versions

## Future Enhancements

- Add GET /ingredients/{id} endpoint
- Add DELETE /ingredients/{id} endpoint
- Implement persistent storage (SQLite, PostgreSQL)
- Add price history endpoints as defined in the OpenAPI spec
- Add pagination for list endpoints
- Add filtering and sorting options
- Add OpenAPI/Swagger UI integration

## License

This project is part of the recipes and cost calculator application.

## Contributing

This is a private project. For issues or suggestions, please open an issue on GitHub.
