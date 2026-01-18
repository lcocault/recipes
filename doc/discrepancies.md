# Discrepancies Between PlantUML Model and PRD

## Ingredient Model
- **PlantUML:** Includes `density` and `notes` as optional fields.
- **PRD:** Matches this structure.

## IngredientPriceHistory
- **PlantUML:** Includes `pricePerPurchaseUnit` and `effectiveDate`.
- **PRD:** Matches this structure.

## Recipe Model
- **PlantUML:** Includes `yieldFactor` and `notes` as optional fields.
- **PRD:** Matches this structure.

## RecipeIngredient
- **PlantUML:** Includes `yieldLossPercent` as an optional field.
- **PRD:** Matches this structure.

## RecipeCostSnapshot
- **PlantUML:** Includes fields like `ingredientCosts`, `packagingCost`, `laborCost`, `energyCost`, `overhead`, `totalCost`, and `perPortionCost`.
- **PRD:** Matches this structure.

## Packaging
- **PlantUML:** Not explicitly modeled.
- **PRD:** Includes a detailed description of packaging, including `packagingCost` and `packaging catalog`.

## SalesRecord
- **PlantUML:** Includes fields like `soldAt`, `quantity`, and `salePricePerPortion`.
- **PRD:** Not explicitly mentioned in the MVP scope.

## Relationships
- **PlantUML:** Explicitly models relationships between entities.
- **PRD:** Describes relationships implicitly but does not use UML-style notation.

## Recommendations
1. Decide whether to add the `SalesRecord` entity to the PRD or remove it from the PlantUML model for consistency.
2. Determine if the `Packaging` entity should be added to the PlantUML model.
3. Update the PRD to include UML-style relationships for clarity, if desired.