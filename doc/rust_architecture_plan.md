# Architecture Plan for Recipes & Cost Calculator in Rust

## Overview
This document outlines the architecture for implementing the Recipes & Cost Calculator application entirely in Rust. The architecture leverages Rust's performance, safety, and concurrency features to build a robust and efficient system.

## Key Components

### 1. Backend
- **Framework:** Axum (or Actix-Web as an alternative)
  - Provides a lightweight and performant HTTP server for building RESTful APIs.
- **Database:** SQLx (or Diesel as an alternative)
  - SQLx offers async database interactions with compile-time query validation.
- **Features:**
  - Ingredient Management: CRUD operations for ingredients and price history.
  - Recipe Management: CRUD operations for recipes and associated ingredients.
  - Cost Calculation: Compute total costs, including labor, energy, packaging, and overhead.
  - Sales Tracking: Record sales data and calculate margins.

### 2. Frontend
- **Framework:** Yew (or Leptos/Sycamore as alternatives)
  - A Rust-based framework for building web frontends that compile to WebAssembly (Wasm).
- **Features:**
  - Recipe Form: Create and edit recipes with live cost previews.
  - Ingredient Catalog: Manage ingredients and update prices.
  - Cost Preview: Display cost breakdowns and margins.

### 3. Data Flow
1. **Frontend:**
   - Users interact with the web interface to manage ingredients, recipes, and sales data.
   - The frontend communicates with the backend via RESTful API endpoints.

2. **Backend:**
   - Handles API requests, performs business logic, and interacts with the database.
   - Ensures data integrity and performs cost calculations.

3. **Database:**
   - Stores data for ingredients, recipes, costs, and sales.
   - Provides historical data for reproducibility.

### 4. Deployment
- **Containerization:** Use Docker to containerize the application for consistent deployment.
- **Hosting:** Deploy on platforms like AWS, DigitalOcean, or Fly.io.
- **CI/CD:** Use GitHub Actions for automated testing and deployment.

## Technology Stack
- **Backend:** Axum, SQLx, Tokio (for async runtime)
- **Frontend:** Yew (compiled to WebAssembly)
- **Database:** PostgreSQL (or SQLite for local development)
- **Testing:** Cargo test for unit and integration tests
- **Build Tools:** Cargo for dependency management and builds

## Advantages of Rust
- **Performance:** Rust's zero-cost abstractions ensure high performance.
- **Safety:** The ownership model prevents common bugs like null pointer dereferences and data races.
- **Concurrency:** Async support enables efficient handling of multiple requests.
- **Unified Language:** Using Rust for both frontend and backend simplifies the tech stack.

## Next Steps
1. Set up the project structure with Axum for the backend and Yew for the frontend.
2. Define the database schema and implement migrations using SQLx.
3. Develop API endpoints for ingredient and recipe management.
4. Build the frontend components for recipe and ingredient management.
5. Implement cost calculation logic and integrate it with the frontend.
6. Add unit and integration tests to ensure reliability.

## UML Diagram
A PlantUML diagram can be created to visualize the architecture. Let me know if you would like me to generate it.