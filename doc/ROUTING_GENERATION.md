# REST API Routing Code Generation

## Overview

This project uses build-time code generation to automatically create REST API route definitions from the OpenAPI specification (`doc/openapi.yaml`). This approach ensures that the API implementation stays in sync with its specification and reduces manual maintenance.

## How It Works

### Build Script (`build.rs`)

The `build.rs` script runs during the Cargo build process before the main compilation:

1. **Parse OpenAPI Spec**: Reads and parses `doc/openapi.yaml` using `serde_yaml`
2. **Extract Routes**: Extracts path definitions, HTTP methods, and operation metadata
3. **Generate Code**: Creates Rust code that constructs an Axum `Router` with all defined routes
4. **Write Output**: Saves the generated code to `$OUT_DIR/generated_routes.rs`
5. **Auto-rebuild**: Triggers rebuild whenever `doc/openapi.yaml` changes

### Generated Code Structure

The generated file (`generated_routes.rs`) contains:
- A public `build_router(state: AppState) -> Router` function
- Route mappings for all paths defined in the OpenAPI spec
- Proper method chaining for multiple HTTP methods on the same path

Example generated code:
```rust
// This file is auto-generated from doc/openapi.yaml by build.rs
// DO NOT EDIT MANUALLY

use axum::{routing::{get, post, put, delete, patch}, Router};

pub fn build_router(state: crate::handlers::AppState) -> Router {
    Router::new()
        .route("/ingredients", get(crate::handlers::ingredients::list_ingredients)
                                .post(crate::handlers::ingredients::create_ingredient))
        .route("/ingredients/:id", put(crate::handlers::ingredients::update_ingredient))
        .route("/recipes", get(crate::handlers::recipes::list_recipes)
                            .post(crate::handlers::recipes::create_recipe))
        .route("/recipes/:id", put(crate::handlers::recipes::update_recipe))
        .with_state(state)
}
```

## Adding New Routes

To add a new API endpoint:

1. **Update OpenAPI Spec**: Add the new path and methods to `doc/openapi.yaml`
   ```yaml
   paths:
     /new-resource:
       get:
         summary: Get all items
         responses:
           '200':
             description: List of items
       post:
         summary: Create a new item
         responses:
           '201':
             description: Item created
   ```

2. **Run Build**: The build script will automatically generate the route mapping
   ```bash
   cargo build
   ```

3. **Implement Handler**: Create the corresponding handler function
   - Handler functions must be placed in the appropriate module under `src/handlers/`
   - The naming convention is: `{module}::{action}_{resource}`

## Handler Naming Conventions

The build script automatically derives handler names from the API path and HTTP method:

| Path | Method | Generated Handler Name |
|------|--------|------------------------|
| `/ingredients` | GET | `ingredients::list_ingredients` |
| `/ingredients` | POST | `ingredients::create_ingredient` |
| `/ingredients/{id}` | GET | `ingredients::get_ingredient` |
| `/ingredients/{id}` | PUT | `ingredients::update_ingredient` |
| `/ingredients/{id}` | DELETE | `ingredients::delete_ingredient` |
| `/recipes` | GET | `recipes::list_recipes` |
| `/recipes` | POST | `recipes::create_recipe` |

### Naming Rules:
- **List operations** (GET without ID): `list_{plural_resource}`
- **Get operations** (GET with ID): `get_{singular_resource}`
- **Create operations** (POST): `create_{singular_resource}`
- **Update operations** (PUT): `update_{singular_resource}`
- **Delete operations** (DELETE): `delete_{singular_resource}`

## Path Parameter Conversion

OpenAPI path parameters are automatically converted to Axum syntax:
- OpenAPI: `/ingredients/{id}`
- Axum: `/ingredients/:id`

## Debugging Generated Code

If you need to inspect the generated routing code:

1. **Build the project**: `cargo build`
2. **Locate the generated file**: The file is in `target/debug/build/recipes-api-*/out/generated_routes.rs`
3. **View it**:
   ```bash
   find target/debug/build -name "generated_routes.rs" | head -1 | xargs cat
   ```

## Development Workflow

### Making Changes

1. Edit `doc/openapi.yaml` to add/modify routes
2. Run `cargo build` to regenerate routes
3. Implement or update handler functions
4. Test your changes: `cargo test`
5. Run the server: `cargo run`

### Testing

The existing test suite validates that:
- All generated routes work correctly
- Handler functions are called with proper parameters
- Request/response flow is correct

Run tests with:
```bash
cargo test
```

## File Structure

```
├── build.rs                      # Build script for code generation
├── doc/
│   ├── openapi.yaml             # OpenAPI specification (source of truth)
│   └── ROUTING_GENERATION.md    # This file
├── src/
│   ├── main.rs                  # Includes and uses generated routes
│   └── handlers/
│       ├── mod.rs
│       ├── ingredients.rs       # Handler implementations
│       └── recipes.rs           # Handler implementations
└── target/
    └── debug/
        └── build/
            └── recipes-api-*/
                └── out/
                    └── generated_routes.rs  # Generated routing code (not in git)
```

## Troubleshooting

### Build Errors

**Error: "cannot find function `X` in module `Y`"**
- Solution: Implement the missing handler function in the appropriate module
- The function name should match the naming convention described above

**Error: "failed to parse OpenAPI spec"**
- Solution: Validate your YAML syntax in `doc/openapi.yaml`
- Ensure all required fields are present

### Route Not Working

1. Verify the route is in `doc/openapi.yaml`
2. Run `cargo clean && cargo build` to force regeneration
3. Check that the handler function exists and is public
4. Verify the handler function signature matches Axum expectations

## Architecture Benefits

1. **Single Source of Truth**: API contract is defined once in OpenAPI spec
2. **DRY Principle**: Route definitions are not duplicated
3. **Type Safety**: Rust compiler catches missing handlers
4. **Documentation**: OpenAPI spec serves as API documentation
5. **Maintainability**: Changes to routes require only OpenAPI updates

## Future Enhancements

Potential improvements (not currently implemented):
- Generate handler stubs for new routes automatically
- Generate request/response types from OpenAPI schemas
- Validate at compile time that all handlers exist
- Use OpenAPI `operationId` to customize handler names
- Support for middleware configuration per route
