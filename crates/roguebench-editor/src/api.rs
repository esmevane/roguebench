//! REST API handlers for item management.

use axum::{
    Json,
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use roguebench_core::items::ItemDefinition;
use serde::{Deserialize, Serialize};

use crate::db::{DbError, EditorDb};

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: EditorDb,
}

/// API error response.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl From<DbError> for (StatusCode, Json<ErrorResponse>) {
    fn from(err: DbError) -> Self {
        let (status, message) = match &err {
            DbError::NotFound(id) => (StatusCode::NOT_FOUND, format!("Item not found: {}", id)),
            DbError::Sqlite(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            DbError::Serialization(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            DbError::Lock => (StatusCode::INTERNAL_SERVER_ERROR, "Database lock error".to_string()),
        };
        (status, Json(ErrorResponse { error: message }))
    }
}

/// Request body for creating/updating items.
#[derive(Debug, Deserialize)]
pub struct ItemRequest {
    #[serde(flatten)]
    pub item: ItemDefinition,
}

/// Response for validation endpoint.
#[derive(Serialize, Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub errors: Vec<String>,
}

/// Create the API router.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/items", get(list_items))
        .route("/api/items", post(create_item))
        .route("/api/items/{id}", get(get_item))
        .route("/api/items/{id}", put(update_item))
        .route("/api/items/{id}", delete(delete_item))
        .route("/api/validate/item", post(validate_item))
        .with_state(state)
}

/// GET /api/items - List all items.
async fn list_items(
    State(state): State<AppState>,
) -> Result<Json<Vec<ItemDefinition>>, (StatusCode, Json<ErrorResponse>)> {
    let items = state.db.list_items()?;
    Ok(Json(items))
}

/// GET /api/items/:id - Get a specific item.
async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ItemDefinition>, (StatusCode, Json<ErrorResponse>)> {
    let item = state.db.get_item(&id)?;
    Ok(Json(item))
}

/// POST /api/items - Create a new item.
async fn create_item(
    State(state): State<AppState>,
    Json(request): Json<ItemRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    state.db.create_item(&request.item)?;
    Ok((StatusCode::CREATED, Json(request.item)))
}

/// PUT /api/items/:id - Update an existing item.
async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<ItemRequest>,
) -> Result<Json<ItemDefinition>, (StatusCode, Json<ErrorResponse>)> {
    state.db.update_item(&id, &request.item)?;
    Ok(Json(request.item))
}

/// DELETE /api/items/:id - Delete an item.
async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let deleted = state.db.delete_item(&id)?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, Json(ErrorResponse {
            error: format!("Item not found: {}", id),
        })))
    }
}

/// POST /api/validate/item - Validate an item without saving.
async fn validate_item(
    Json(request): Json<ItemRequest>,
) -> Json<ValidationResponse> {
    let mut errors = Vec::new();

    // Validate required fields
    if request.item.id.0.is_empty() {
        errors.push("ID is required".to_string());
    }
    if request.item.name.is_empty() {
        errors.push("Name is required".to_string());
    }

    // Validate ID format (alphanumeric + underscore)
    if !request.item.id.0.chars().all(|c| c.is_alphanumeric() || c == '_') {
        errors.push("ID must contain only alphanumeric characters and underscores".to_string());
    }

    Json(ValidationResponse {
        valid: errors.is_empty(),
        errors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, Method};
    use http_body_util::BodyExt;
    use roguebench_core::items::ItemType;
    use tower::ServiceExt;

    fn test_app() -> Router {
        let db = EditorDb::in_memory().unwrap();
        let state = AppState { db };
        router(state)
    }

    #[tokio::test]
    async fn list_empty_items() {
        let app = test_app();

        let response = app
            .oneshot(Request::builder().uri("/api/items").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let items: Vec<ItemDefinition> = serde_json::from_slice(&body).unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn create_and_get_item() {
        let db = EditorDb::in_memory().unwrap();
        let state = AppState { db: db.clone() };
        let app = router(state);

        let item = ItemDefinition::new("test_sword", "Test Sword", ItemType::Equipment);
        let body = serde_json::to_string(&item).unwrap();

        // Create
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/items")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        // Get
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/items/test_sword")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let loaded: ItemDefinition = serde_json::from_slice(&body).unwrap();
        assert_eq!(loaded.name, "Test Sword");
    }

    #[tokio::test]
    async fn item_not_found() {
        let app = test_app();

        let response = app
            .oneshot(Request::builder().uri("/api/items/nonexistent").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn validate_item() {
        let app = test_app();

        let item = ItemDefinition::new("valid_item", "Valid Item", ItemType::Consumable);
        let body = serde_json::to_string(&item).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/validate/item")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let result: ValidationResponse = serde_json::from_slice(&body).unwrap();
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }
}
