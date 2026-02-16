---
name: api-doc
description: Generate or update OpenAPI spec and API reference documentation from the current backend codebase.
context: fork
agent: Explore
---

# Generate API Documentation

Scan the backend codebase and generate/update API documentation.

## Steps

1. Scan all route handlers in `backend/crates/api/src/`:
   - Find all registered routes (method + path)
   - Read handler function signatures
   - Extract request/response types

2. Read type definitions from `backend/crates/core/src/types/`

3. Update `shared/openapi.yaml`:
   - Add/update each endpoint
   - Include request body schemas
   - Include response schemas
   - Add authentication requirements
   - Add error response schemas

4. Generate `docs/api-reference.md`:
   - Group endpoints by resource (products, cart, orders, auth, etc.)
   - For each endpoint: method, path, description, auth required, request example, response example
   - Include error codes and their meanings

## OpenAPI Conventions
- Use components/schemas for reusable types
- Include examples for each schema
- Document pagination parameters
- Document filter/sort query parameters
