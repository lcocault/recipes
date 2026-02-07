# Copilot Instructions for the recipes project

Purpose: provide concise, project-level guidance Copilot should always apply when generating code, tests, docs, and PR suggestions.

Copy this short block into your Copilot custom instructions (or keep this file in-repo as the canonical source):

Project: Rust recipes API — small, API-first service for managing recipes and ingredients.
Prefer: idiomatic Rust (2021/2024), readable code, small focused functions, explicit error handling with `thiserror`/`anyhow`.
Frameworks: use `axum` for HTTP handlers, `serde` for JSON, `tokio` for async runtime.
Testing: include unit tests; use `#[cfg(test)]` with `tokio::test` for async functions; keep tests deterministic and minimal.
Style: snake_case for functions, PascalCase for types, public APIs documented with doc comments; keep functions < 60 lines.
Constraints: avoid `unsafe`; avoid large new runtime dependencies without justification; prefer small, audited crates.

Architecture: prefer Hexagonal (Ports & Adapters) — keep domain logic in a `core` or `domain` module/crate without framework types; expose ports as traits under `core::ports`; implement adapters (HTTP, DB, persistence) under `adapters/*`; composition/wiring belongs only in `main.rs` or an `app::composition_root`.

Practical rules:
- Domain: put business rules and types in `core`/`domain`; do not reference `axum`, `tokio`, `serde`, or other infra crates from domain code.
- Ports: define traits for persistence and external APIs in `core::ports`.
- Adapters: implement HTTP, DB, and other infra in `adapters::{http,persistence}` and depend on the port traits.
- Wiring: perform wiring/composition only in the application root (e.g., `main.rs`).
- Tests: domain unit tests must not require runtime or infra; integration tests may exercise adapters.

Notes & usage
- Keep this file at the repo root so teammates can reference it.
- Short is better: keep the Copilot-setting copy to 4–8 lines (the block above).
- When opening Copilot Chat or asking for code, reference the instruction implicitly; pair it with a `Skill` (prompt template) for repeatable tasks.

Examples (one-line copies you can paste into Copilot settings)
- "Project: Rust recipes API — idiomatic Rust 2021; use axum, serde, tokio; explicit errors with thiserror; no unsafe; include unit tests."

When to update
- Change this file by PR when team conventions evolve (new framework, different error strategy, new banned crates).

Want me to also create editor snippets that insert popular `Skills` (add endpoint, write test, refactor)? Reply with "snippets" and I'll add `.vscode/copilot-skills.code-snippets`.
