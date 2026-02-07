# Epics and User Stories for Recipes & Cost Calculator

## Epic 1: Ingredient Management
### User Stories:
1. **As a user, I want to add new ingredients to the catalog so that I can use them in recipes.**
   - Acceptance Criteria:
     - The system allows input of ingredient name, default unit, purchase unit, and price.
     - The system validates required fields.

2. **As a user, I want to update ingredient prices with effective dates so that I can maintain accurate cost calculations.**
   - Acceptance Criteria:
     - The system allows adding new price entries with timestamps.
     - Historical prices are preserved.

3. **As a user, I want to view a list of all ingredients so that I can manage my catalog.**
   - Acceptance Criteria:
     - The system displays ingredient details in a table.
     - The table supports sorting and filtering.

4. **As a developer, I want to implement a search feature for ingredients so that users can quickly find specific items in the catalog.**
   - Acceptance Criteria:
     - The system provides a search bar for ingredient names.
     - Search results are displayed dynamically as the user types.

## Epic 2: Recipe Management
### User Stories:
1. **As a user, I want to create new recipes by selecting ingredients and specifying quantities so that I can calculate costs.**
   - Acceptance Criteria:
     - The system allows selecting ingredients from the catalog.
     - The system supports input of quantities and units.

2. **As a user, I want to edit existing recipes so that I can make changes as needed.**
   - Acceptance Criteria:
     - The system allows updating ingredient lists and quantities.
     - Changes are saved persistently.

3. **As a user, I want to delete recipes so that I can remove outdated entries.**
   - Acceptance Criteria:
     - The system confirms deletion before removing a recipe.

4. **As a user, I want to search for ingredients by name so that I can quickly find specific items in the catalog.**
   - Acceptance Criteria:
     - The system provides a search bar for ingredient names.
     - Search results are displayed dynamically as the user types.

## Epic 3: Cost Calculation
### User Stories:
1. **As a user, I want to calculate the total cost of a recipe so that I can determine pricing.**
   - Acceptance Criteria:
     - The system calculates ingredient costs based on the latest prices.
     - The system includes labor, energy, packaging, and overhead costs.

2. **As a user, I want to view a cost breakdown for each recipe so that I can understand the cost structure.**
   - Acceptance Criteria:
     - The system displays costs for ingredients, labor, energy, packaging, and overhead.
     - The system calculates per-portion costs.

---

## Epic 4: Sales Tracking
### User Stories:
1. **As a user, I want to record sales data for recipes so that I can track revenue and margins.**
   - Acceptance Criteria:
     - The system allows input of sale date, quantity, and price per portion.
     - Sales data is linked to recipes.

2. **As a user, I want to view sales reports so that I can analyze performance over time.**
   - Acceptance Criteria:
     - The system generates reports showing revenue, margins, and best-selling recipes.

---

## Epic 5: User Interface
### User Stories:
1. **As a user, I want a simple and intuitive interface for managing recipes and ingredients so that I can work efficiently.**
   - Acceptance Criteria:
     - The interface is responsive and mobile-friendly.
     - Forms are easy to navigate and validate inputs.

2. **As a user, I want live cost previews while editing recipes so that I can see the impact of changes immediately.**
   - Acceptance Criteria:
     - The system updates cost calculations in real-time as inputs change.

---

## Epic 6: Deployment and Maintenance
### User Stories:
1. **As a developer, I want the application to be containerized so that it can be deployed consistently.**
   - Acceptance Criteria:
     - The application runs in Docker containers.
     - Environment variables are configurable.

2. **As a developer, I want automated tests for critical features so that I can ensure reliability.**
   - Acceptance Criteria:
     - Unit and integration tests cover key functionality.
     - Tests are run automatically in CI/CD pipelines.