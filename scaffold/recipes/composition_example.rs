// Composition example (pseudo-code) showing how to wire the adapters to the domain in the
// application composition root (`main.rs` or `app::composition_root`).

// Example (non-compiling scaffold):
/*
use std::sync::Arc;
use scaffold::recipes::adapters::persistence::InMemoryRepo;
use scaffold::recipes::adapters::http::HttpAdapter;

fn main() {
    // Create adapters
    let repo = Arc::new(InMemoryRepo::new());
    let http = HttpAdapter::new(repo.clone());

    // Build your axum/router, registering handlers that call into `http` which uses ports
    // The important rule: domain types live in `core`, adapters implement `core::ports`,
    // and wiring happens here in main only.
}
*/

// This file is a guide: copy the composition pattern into your real `main.rs`.
