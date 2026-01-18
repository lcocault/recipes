# Architecture Proposal for Recipes & Cost Calculator

## Overview
This document outlines the proposed architecture for the Recipes & Cost Calculator application. The architecture is designed to meet the requirements outlined in the PRD and align with the updated PlantUML model.

## Key Components

### 1. Ingredient Reference System (IRS)
- **Purpose:** Manage ingredient data, including types, units, and price history.
- **Entities:**
  - `Ingredient`: Stores details like name, default unit, purchase unit, and optional density.
  - `IngredientType`: Enum for categorizing ingredients (e.g., ENERGY, FOOD).
  - `IngredientPriceHistory`: Tracks price changes over time.
- **Relationships:**
  - `Ingredient` has a one-to-many relationship with `IngredientPriceHistory`.

### 2. Recipe Management System (RMS)
- **Purpose:** Handle recipe creation, updates, and ingredient associations.
- **Entities:**
  - `Recipe`: Stores recipe details, including title, servings, and steps.
  - `RecipeIngredient`: Links recipes to ingredients with quantities and optional yield loss.
- **Relationships:**
  - `Recipe` has a one-to-many relationship with `RecipeIngredient`.
  - `RecipeIngredient` references `Ingredient`.

### 3. Cost/History Management System (CHMS)
- **Purpose:** Calculate and store cost snapshots and sales records.
- **Entities:**
  - `RecipeCostSnapshot`: Stores calculated costs for a recipe at a specific time.
  - `SalesRecord`: Tracks sales data, including quantity and price per portion.
- **Relationships:**
  - `Recipe` has a one-to-many relationship with `RecipeCostSnapshot`.
  - `Recipe` has a one-to-many relationship with `SalesRecord`.

### 4. Packaging System
- **Purpose:** Manage packaging data and costs.
- **Entities:**
  - `Packaging`: Stores details like name, unit, and price.
- **Relationships:**
  - `Recipe` optionally references `Packaging` for per-portion packaging costs.

## Data Flow
1. **Ingredient Management:**
   - Users add or update ingredients and their price history.
   - The system ensures price history is timestamped and linked to the correct ingredient.

2. **Recipe Management:**
   - Users create or update recipes by selecting ingredients and specifying quantities.
   - The system calculates ingredient costs using the latest price or a user-specified date.

3. **Cost Calculation:**
   - The system calculates total costs, including labor, energy, packaging, and overhead.
   - Costs are stored as snapshots for reproducibility.

4. **Sales Tracking:**
   - Users record sales data, which is linked to recipes.
   - The system calculates margins and tracks revenue over time.

## Technology Stack
- **Backend:** Node.js with SQLite for persistence.
- **Frontend:** React or Svelte for a responsive user interface.
- **Diagram Generation:** PlantUML for visualizing relationships.

## UML Diagram
Refer to the `doc/model.puml` file for the updated UML diagram representing the concepts pf the application.

## Next Steps
1. Confirm the proposed architecture aligns with business needs.
2. Begin implementation by scaffolding backend models and APIs.
3. Develop frontend components for ingredient and recipe management.