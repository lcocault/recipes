# COPILOT_SKILLS.md — reusable Copilot prompt templates for the recipes repo

Purpose: store short, parameterized prompt templates ("Skills") the team can reuse when asking Copilot to perform common tasks. Keep each skill simple, example-driven, and constrained by the project `Instructions`.

Format (human-readable)
- Name: short id
- Intent: one-line goal
- Prompt: the text to send to Copilot with `{placeholders}`
- Example: concrete values showing expected input

Skills

- Name: add_endpoint
  Intent: Add a new HTTP endpoint
  Prompt: |
    Create a new async `{method}` endpoint at `{path}` that accepts `{request_schema}` and returns `{response_schema}` as JSON. Use the project's frameworks (`axum`, `serde`, `tokio`) and follow project instructions: idiomatic Rust, explicit errors with `thiserror`/`anyhow`, no `unsafe`. Include input validation, proper status codes, and a unit test (success + error case).
  Example: method=POST path=/recipes request_schema=CreateRecipeRequest response_schema=Recipe

- Name: write_unit_test
  Intent: Add unit tests for a function
  Prompt: |
    Write unit tests for `{function_name}` in module `{module_path}`. Use `#[cfg(test)]` and `tokio::test` if async. Provide at least one success case and one error/boundary case. Keep tests deterministic and minimal.
  Example: function_name=calculate_nutritional_info module_path=src/ingredient/mod.rs

- Name: refactor_small_fn
  Intent: Break a long function into helpers
  Prompt: |
    Refactor `{function_name}` in `{module_path}` into smaller private helper functions. Preserve behavior and public API. Add doc comments to new helpers and keep each helper focused (single responsibility).
  Example: function_name=process_recipe module_path=src/recipe/lib.rs

- Name: add_integration_test_stub
  Intent: Add an integration test skeleton
  Prompt: |
    Create an integration test in `tests/{name}.rs` that starts an instance of the HTTP server (using `axum::Router`), runs one request against `{path}`, and asserts the response status and JSON shape. Keep external dependencies mocked or use in-memory stores.
  Example: name=recipes_integration path=/recipes

- Name: scaffold_hexagonal_module
  Intent: Create a hexagonal architecture module skeleton
  Prompt: |
    Scaffold a hexagonal module named `{name}`:
    - Add domain types and business rules in `core::{name}` (keep free of framework/runtime types).
    - Define port traits in `core::ports::{name}` for persistence and external interactions.
    - Add adapter stubs under `adapters::{name}::http` and `adapters::{name}::persistence` that implement the port traits.
    - Add a minimal `README.md` explaining the module's responsibilities.
    - Show wiring/composition code in `main.rs` or `app::composition_root` that composes the adapters with the domain via the ports.
    Use `thiserror`/`anyhow` for errors and include a minimal domain unit test that does not depend on infra.
  Example: name=recipes

Best practices
- Keep prompts short and parameterized; rely on `COPILOT_INSTRUCTIONS.md` for project-wide constraints.
- Include an example with each skill to reduce ambiguity.
- Version changes via small PRs so teammates adopt improvements.

How to use
- Copy the `Prompt` text into Copilot Chat or your editor prompt, replace placeholders, and run.
- Create editor snippets (e.g., `.vscode/copilot-skills.code-snippets`) for frequent skills.

Next steps I can do for you
- Generate a `.vscode` snippet file containing the `add_endpoint`, `write_unit_test`, and `refactor_small_fn` skills.
- Convert these skills into a JSON format for an automation tool or chat macro if you tell me the target format.
