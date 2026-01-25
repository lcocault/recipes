#!/usr/bin/env bash
set -euo pipefail

# Generates Rust code from OpenAPI specs under src/api into src/generated
mkdir -p src/generated
for spec in src/api/*.yaml; do
  name=$(basename "$spec" .yaml)
  echo "Generating Rust client for $spec -> src/generated/$name"
  docker run --rm -v "$(pwd):/local" openapitools/openapi-generator-cli generate \
    -g rust \
    -i /local/$spec \
    -o /local/src/generated/$name \
    --additional-properties=packageName=${name}_api,packageVersion=0.1.0,library=reqwest,supportAsync=true
done

echo "Generation complete"
