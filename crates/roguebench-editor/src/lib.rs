//! Web editor backend for roguebench.
//!
//! Provides an HTTP API and simple HTML interface for content authoring.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use roguebench_core::EntityDef;
use roguebench_protocol::EditorMessage;
use roguebench_storage::ContentStore;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;

/// Configuration for the editor.
pub struct EditorConfig {
    /// Content storage backend.
    pub storage: Arc<dyn ContentStore>,
    /// Sender for engine messages.
    pub message_tx: mpsc::UnboundedSender<EditorMessage>,
    /// Address to listen on.
    pub listen_addr: SocketAddr,
}

/// Shared state for axum handlers.
#[derive(Clone)]
struct AppState {
    store: Arc<dyn ContentStore>,
    message_tx: mpsc::UnboundedSender<EditorMessage>,
}

#[derive(Deserialize)]
struct CreateEntityRequest {
    name: String,
    health: i32,
}

#[derive(Serialize, Deserialize)]
struct EntityResponse {
    id: String,
    name: String,
    health: i32,
}

async fn index() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Roguebench Editor</title>
    <style>
        body { font-family: sans-serif; max-width: 600px; margin: 40px auto; padding: 0 20px; }
        input, button { padding: 8px; margin: 4px 0; }
        input { width: 200px; }
        ul { list-style: none; padding: 0; }
        li { padding: 8px; background: #f0f0f0; margin: 4px 0; }
    </style>
</head>
<body>
    <h1>Entity Editor</h1>
    <form id="create-form">
        <input type="text" id="name" placeholder="Entity name" required>
        <input type="number" id="health" placeholder="Health" value="100" required>
        <button type="submit">Create</button>
    </form>
    <h2>Entities</h2>
    <ul id="entities"></ul>
    <script>
        async function loadEntities() {
            const res = await fetch('/entities');
            const entities = await res.json();
            const ul = document.getElementById('entities');
            ul.innerHTML = entities.map(e => `<li>${e.name} (HP: ${e.health}) - ${e.id}</li>`).join('');
        }
        document.getElementById('create-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const name = document.getElementById('name').value;
            const health = parseInt(document.getElementById('health').value, 10);
            await fetch('/entities', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name, health })
            });
            document.getElementById('name').value = '';
            document.getElementById('health').value = '100';
            loadEntities();
        });
        loadEntities();
    </script>
</body>
</html>"#,
    )
}

async fn list_entities(State(state): State<AppState>) -> impl IntoResponse {
    match state.store.load_entities() {
        Ok(entities) => {
            let response: Vec<EntityResponse> = entities
                .into_iter()
                .map(|e| EntityResponse {
                    id: e.id.to_string(),
                    name: e.name,
                    health: e.health,
                })
                .collect();
            Json(response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn create_entity(
    State(state): State<AppState>,
    Json(req): Json<CreateEntityRequest>,
) -> impl IntoResponse {
    let entity = EntityDef::new(req.name, req.health);
    match state.store.save_entity(&entity) {
        Ok(()) => {
            let _ = state.message_tx.send(EditorMessage::ReloadEntities);
            let response = EntityResponse {
                id: entity.id.to_string(),
                name: entity.name,
                health: entity.health,
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Build the editor router.
pub fn router(storage: Arc<dyn ContentStore>, message_tx: mpsc::UnboundedSender<EditorMessage>) -> Router {
    let state = AppState {
        store: storage,
        message_tx,
    };

    Router::new()
        .route("/", get(index))
        .route("/entities", get(list_entities).post(create_entity))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Run the editor web server.
pub async fn run(config: EditorConfig) {
    let app = router(config.storage, config.message_tx);

    tracing::info!("Web editor listening on http://{}", config.listen_addr);

    let listener = tokio::net::TcpListener::bind(config.listen_addr)
        .await
        .expect("Failed to bind editor address");
    axum::serve(listener, app)
        .await
        .expect("Editor server failed");
}

pub mod prelude {
    pub use crate::{router, run, EditorConfig};
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use roguebench_storage::{ContentStore, MemoryStore};
    use tower::ServiceExt;

    fn test_router() -> (Router, mpsc::UnboundedReceiver<EditorMessage>) {
        let storage = Arc::new(MemoryStore::new());
        let (tx, rx) = mpsc::unbounded_channel();
        (router(storage, tx), rx)
    }

    #[tokio::test]
    async fn index_returns_html() {
        let (app, _rx) = test_router();

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let html = String::from_utf8(body.to_vec()).unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Entity Editor"));
    }

    #[tokio::test]
    async fn list_entities_empty_initially() {
        let (app, _rx) = test_router();

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/entities")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let entities: Vec<EntityResponse> = serde_json::from_slice(&body).unwrap();
        assert!(entities.is_empty());
    }

    #[tokio::test]
    async fn create_entity_returns_201_and_sends_message() {
        let storage: Arc<dyn ContentStore> = Arc::new(MemoryStore::new());
        let (tx, mut rx) = mpsc::unbounded_channel();
        let app = router(Arc::clone(&storage), tx);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/entities")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name": "Goblin", "health": 50}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let created: EntityResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(created.name, "Goblin");
        assert_eq!(created.health, 50);
        assert!(!created.id.is_empty());

        // Verify entity was saved to storage
        let stored = storage.load_entities().unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].name, "Goblin");
        assert_eq!(stored[0].health, 50);

        // Verify message was sent to engine
        let msg = rx.try_recv().unwrap();
        assert!(matches!(msg, EditorMessage::ReloadEntities));
    }

    #[tokio::test]
    async fn list_entities_returns_created_entities() {
        let storage = Arc::new(MemoryStore::new());
        let (tx, _rx) = mpsc::unbounded_channel();

        // Pre-populate storage
        let entity1 = EntityDef::new("Goblin", 30);
        let entity2 = EntityDef::new("Orc", 80);
        storage.save_entity(&entity1).unwrap();
        storage.save_entity(&entity2).unwrap();

        let app = router(storage, tx);

        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/entities")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let entities: Vec<EntityResponse> = serde_json::from_slice(&body).unwrap();
        assert_eq!(entities.len(), 2);

        let names: Vec<&str> = entities.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"Goblin"));
        assert!(names.contains(&"Orc"));

        // Verify health values are returned
        let goblin = entities.iter().find(|e| e.name == "Goblin").unwrap();
        let orc = entities.iter().find(|e| e.name == "Orc").unwrap();
        assert_eq!(goblin.health, 30);
        assert_eq!(orc.health, 80);
    }
}
