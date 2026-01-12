Implementation Tasks — Recipes & Cost Calculator (MVP)

Goal: Convert the PRD MVP items into concrete, actionable tasks with acceptance criteria and rough estimates so we can implement incrementally.

Priority: HIGH = must for MVP, MED = post-MVP but useful, LOW = later.

1) Finalize Units & Conversions (HIGH)
- Task: Define supported units and conversion rules (g, kg, mg, ml, L, piece/unit, tbsp, tsp, cup). Document common densities (e.g., water=1g/ml) and allow custom density per ingredient.
- Acceptance: Document file `/docs/units.md` created; conversion table and examples included.
- Est: 1 day

2) Data Model / Schema (HIGH)
- Task: Produce definitive schema for: Ingredient, IngredientPriceHistory, Packaging, Recipe, RecipeIngredient, RecipeCostSnapshot, SalesRecord.
- Acceptance: `prisma/schema.prisma` or SQL migration file present; sample seed data included.
- Est: 1-2 days

3) Backend scaffold + deps (HIGH)
- Task: Initialize Node.js backend (Express or Fastify), add SQLite + ORM (Prisma recommended), add dev tooling (nodemon, lint). Create basic server and health route.
- Acceptance: `backend/package.json` with scripts, `backend/src/server.ts` (or .js) and `GET /health` route returning 200.
- Est: 0.5 day

4) Models + Migrations + Seed (HIGH)
- Task: Implement DB models per schema and seed with a few sample ingredients and packaging entries (using Excel values if available).
- Acceptance: migrations run successfully; `npm run seed` populates DB with sample data.
- Est: 1 day

5) Cost Calculation Module (HIGH)
- Task: Implement pure JS/TS module `calc/cost.ts` that accepts a recipe object + options (date, laborRate, energyRate, overheadPct) and returns a detailed breakdown (ingredient costs, packaging, labor, energy, overhead, total, per-portion, margin given a price).
- Acceptance: Module exported and covered by unit tests demonstrating conversions, price history lookup, yield/wastage, packaging inclusion.
- Est: 2 days

6) API for Ingredients, Packaging, Recipes, Cost (HIGH)
- Task: Implement REST endpoints:
  - Ingredients: GET/POST/PUT, price history endpoints
  - Packaging: GET/POST/PUT, price history endpoints
  - Recipes: GET/POST/PUT, and GET /recipes/:id/cost
- Acceptance: API documented in `backend/OPENAPI.md` (or simple README) and endpoints respond JSON with expected fields.
- Est: 2 days

7) Unit Tests for Calculations (HIGH)
- Task: Add Jest (or vitest) tests covering calculation correctness and edge cases.
- Acceptance: tests pass in CI locally; at least 90% coverage for calc module logic.
- Est: 1 day

8) Frontend wireframes & minimal UI (MED)
- Task: Create simple React (Vite) or Svelte app with screens: Ingredient catalog, Packaging catalog, Recipe editor (rows), Cost preview panel.
- Acceptance: Local frontend runs and can call backend `GET /recipes/:id/cost` to display breakdown.
- Est: 2-3 days

9) Frontend forms & live preview (MED)
- Task: Implement recipe CRUD forms with ingredient autocomplete, packaging selector per recipe, and a live cost preview panel that calls the cost API on change (debounced).
- Acceptance: Seller can create a recipe and immediately see cost breakdown; basic validation present.
- Est: 3 days

10) CSV import for price updates (MED)
- Task: Implement CSV import for ingredient/packaging price updates (effective dates).
- Acceptance: Admin can POST CSV to `POST /import/prices` and DB updates price history.
- Est: 2 days

11) Sales tracking & basic reports (MED)
- Task: Create SalesRecord model and endpoints to record sales and retrieve basic margin/revenue reports.
- Acceptance: Able to record sales and return aggregated monthly revenue and margin per recipe.
- Est: 2 days

12) Docs, README, Runbook (MED)
- Task: Document setup, run steps, data model, API examples, and how to import price CSVs.
- Acceptance: `README.md` in project root with clear dev and deploy instructions.
- Est: 1 day

13) Deploy MVP (LOW)
- Task: Prepare a simple deployment (e.g., Render or small VPS) with SQLite or managed DB and backups.
- Acceptance: App running on a public URL with basic TLS.
- Est: 1 day

---
Next suggested action: I can scaffold the backend (task 3+4+5) now: initialize Node, add Prisma + SQLite, and implement the cost-calculation module with unit tests. Should I proceed with that? (reply with B to scaffold backend, or C to scaffold frontend UI)
Implementation Tasks — Recipes & Cost Calculator (MVP)

Goal: Convert the PRD MVP items into concrete, actionable tasks with acceptance criteria and rough estimates so we can implement incrementally.

Priority: HIGH = must for MVP, MED = post-MVP but useful, LOW = later.

1) Finalize Units & Conversions (HIGH)
- Task: Define supported units and conversion rules (g, kg, mg, ml, L, piece/unit, tbsp, tsp, cup). Document common densities (e.g., water=1g/ml) and allow custom density per ingredient.
- Acceptance: Document file `/docs/units.md` created; conversion table and examples included.
- Est: 1 day

2) Data Model / Schema (HIGH)
- Task: Produce definitive schema for: Ingredient, IngredientPriceHistory, Packaging, Recipe, RecipeIngredient, RecipeCostSnapshot, SalesRecord.
- Acceptance: `prisma/schema.prisma` or SQL migration file present; sample seed data included.
- Est: 1-2 days

3) Backend scaffold + deps (HIGH)
- Task: Initialize Node.js backend (Express or Fastify), add SQLite + ORM (Prisma recommended), add dev tooling (nodemon, lint). Create basic server and health route.
- Acceptance: `backend/package.json` with scripts, `backend/src/server.ts` (or .js) and `GET /health` route returning 200.
- Est: 0.5 day

4) Models + Migrations + Seed (HIGH)
- Task: Implement DB models per schema and seed with a few sample ingredients and packaging entries (using Excel values if available).
- Acceptance: migrations run successfully; `npm run seed` populates DB with sample data.
- Est: 1 day

5) Cost Calculation Module (HIGH)
- Task: Implement pure JS/TS module `calc/cost.ts` that accepts a recipe object + options (date, laborRate, energyRate, overheadPct) and returns a detailed breakdown (ingredient costs, packaging, labor, energy, overhead, total, per-portion, margin given a price).
- Acceptance: Module exported and covered by unit tests demonstrating conversions, price history lookup, yield/wastage, packaging inclusion.
- Est: 2 days

6) API for Ingredients, Packaging, Recipes, Cost (HIGH)
- Task: Implement REST endpoints:
  - Ingredients: GET/POST/PUT, price history endpoints
  - Packaging: GET/POST/PUT, price history endpoints
  - Recipes: GET/POST/PUT, and GET /recipes/:id/cost
- Acceptance: API documented in `backend/OPENAPI.md` (or simple README) and endpoints respond JSON with expected fields.
- Est: 2 days

7) Unit Tests for Calculations (HIGH)
- Task: Add Jest (or vitest) tests covering calculation correctness and edge cases.
- Acceptance: tests pass in CI locally; at least 90% coverage for calc module logic.
- Est: 1 day

8) Frontend wireframes & minimal UI (MED)
- Task: Create simple React (Vite) or Svelte app with screens: Ingredient catalog, Packaging catalog, Recipe editor (rows), Cost preview panel.
- Acceptance: Local frontend runs and can call backend `GET /recipes/:id/cost` to display breakdown.
- Est: 2-3 days

9) Frontend forms & live preview (MED)
- Task: Implement recipe CRUD forms with ingredient autocomplete, packaging selector per recipe, and a live cost preview panel that calls the cost API on change (debounced).
- Acceptance: Seller can create a recipe and immediately see cost breakdown; basic validation present.
- Est: 3 days

10) CSV import for price updates (MED)
- Task: Implement CSV import for ingredient/packaging price updates (effective dates).
- Acceptance: Admin can POST CSV to `POST /import/prices` and DB updates price history.
- Est: 2 days

11) Sales tracking & basic reports (MED)
- Task: Create SalesRecord model and endpoints to record sales and retrieve basic margin/revenue reports.
- Acceptance: Able to record sales and return aggregated monthly revenue and margin per recipe.
- Est: 2 days

12) Docs, README, Runbook (MED)
- Task: Document setup, run steps, data model, API examples, and how to import price CSVs.
- Acceptance: `README.md` in project root with clear dev and deploy instructions.
- Est: 1 day

13) Deploy MVP (LOW)
- Task: Prepare a simple deployment (e.g., Render or small VPS) with SQLite or managed DB and backups.
- Acceptance: App running on a public URL with basic TLS.
- Est: 1 day

---
Next suggested action: I can scaffold the backend (task 3+4+5) now: initialize Node, add Prisma + SQLite, and implement the cost-calculation module with unit tests. Should I proceed with that? (reply with B to scaffold backend, or C to scaffold frontend UI)
