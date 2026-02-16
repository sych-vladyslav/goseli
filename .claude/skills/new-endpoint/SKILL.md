---
name: new-endpoint
description: Scaffold a new REST API endpoint with handler, route, request/response types, and test stub. Follows project conventions.
disable-model-invocation: true
argument-hint: [METHOD /path description]
---

# Scaffold New API Endpoint

Create a new REST API endpoint following Goseli conventions.

Arguments: $ARGUMENTS (e.g., "GET /api/products List all products")

## Steps

1. Parse arguments: METHOD, path, description

2. Create/update route registration in `backend/crates/api/src/routes/`:
   - Add the route to the appropriate router module
   - Follow existing naming patterns

3. Create handler function in `backend/crates/api/src/handlers/`:
   ```rust
   pub async fn handler_name(
       State(state): State<AppState>,
       // Path/Query/Json extractors as needed
   ) -> Result<Json<ResponseType>, ApiError> {
       // Implementation
   }
   ```

4. Create request/response types in `backend/crates/core/src/types/`:
   - Use serde Serialize/Deserialize
   - Add validation where needed

5. Create SQL query in `backend/crates/db/src/`:
   - Use sqlx query macros
   - Parameterized queries only

6. Create test stub in `backend/tests/`:
   ```rust
   #[tokio::test]
   async fn test_endpoint_name() {
       // TODO: implement test
   }
   ```

## Conventions
- All handlers return `Result<Json<T>, ApiError>`
- Use extractors: Path, Query, Json, State
- Pagination: `?page=1&per_page=20` -> `PaginatedResponse<T>`
- Errors: use the shared ApiError type
- Auth-required endpoints use the `auth` middleware
