Scaffold: Hexagonal `recipes` module

This folder contains a non-invasive scaffold demonstrating a Hexagonal (Ports & Adapters) layout for a `recipes` feature.

Layout
- `core/` — domain types, ports (traits), and domain errors. No framework or runtime types here.
- `adapters/` — infra implementations (HTTP handlers, persistence adapters) that depend on `core::ports`.
- `composition_example.rs` — a small example showing how to wire adapters to the domain in the app root.

This scaffold is intentionally self-contained and safe to keep in the repo (it doesn't modify `src/`), so you can inspect and copy components into your real codebase.
