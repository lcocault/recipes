# Refactor: Hexagonal Architecture Implementation

## Summary
This PR refactors the repository to follow Hexagonal (Ports & Adapters) architecture principles, separating domain logic from infrastructure concerns.

## Changes Made

### New Structure
- **`src/core/`** — Domain types and business logic (framework-agnostic)
  - `ingredient.rs` — Domain `Ingredient` type (moved from `src/ingredient/mod.rs`)
  
- **`src/adapters/`** — Infrastructure implementations
  - `http.rs` — HTTP handlers using axum (moved from `src/ingredient/api.rs`)
  - `persistence.rs` — In-memory repository adapter

### Modified Files
- **`src/lib.rs`** — Now exports `core` and `adapters` modules
- **`src/ingredient/lib.rs`** — Compatibility shim re-exporting `core::ingredient`
- **`src/ingredient/main.rs`** — Updated import to use `recipes::core::Ingredient`
- **`Cargo.toml`** — Updated `api_server` binary path to `src/bin/api_server.rs`
- **`src/bin/api_server.rs`** — New location for HTTP server binary (currently a scaffold)

### Deleted Files
- `src/ingredient/mod.rs` — Moved to `src/core/ingredient.rs`
- `src/ingredient/api.rs` — Logic moved to `src/adapters/http.rs`

### Documentation Added
- **`COPILOT_INSTRUCTIONS.md`** — Project-wide Copilot instructions including hexagonal architecture guidelines
- **`COPILOT_SKILLS.md`** — Reusable prompt templates for common development tasks
- **`scaffold/recipes/`** — Complete hexagonal architecture example with ports, adapters, and composition

## Architecture Benefits
1. **Testability** — Domain logic in `core` can be tested without infrastructure dependencies
2. **Flexibility** — Easy to swap adapters (e.g., replace in-memory storage with PostgreSQL)
3. **Maintainability** — Clear separation of concerns between business rules and technical implementation
4. **Framework Independence** — Domain code doesn't depend on axum, tokio, or serde

## Build Status
- ✅ `cargo check` passes (1 minor warning about private type visibility)
- ✅ `cargo test` passes (all existing tests continue to work)
- ✅ No breaking changes to public API

## Next Steps
1. Add port traits (repository interfaces) to `src/core/ports.rs`
2. Implement adapters that satisfy the port traits
3. Wire adapters in application composition root
4. Add integration tests using the new structure

## Migration Guide for Team
- Old: `use recipes::ingredient::Ingredient;`
- New: `use recipes::core::Ingredient;` (compatibility shim maintains old imports)

The `scaffold/recipes/` directory contains a complete working example demonstrating the architecture pattern.
