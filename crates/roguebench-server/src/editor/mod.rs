//! Web editor module
//!
//! Serves HTML forms at localhost:8080 for content authoring.
//! "Functional but not fancy" - simple forms that work.

mod enemies;
mod html;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

/// Run the web editor server
pub async fn run_editor(state: AppState) -> anyhow::Result<()> {
    let app = Router::new()
        // Home page
        .route("/", get(index))
        // Enemy routes
        .route("/enemies", get(enemies::list_enemies))
        .route("/enemies/new", get(enemies::new_enemy_form))
        .route("/enemies", post(enemies::create_enemy))
        .route("/enemies/{id}", get(enemies::show_enemy))
        .route("/enemies/{id}/edit", get(enemies::edit_enemy_form))
        .route("/enemies/{id}", post(enemies::update_enemy))
        .route("/enemies/{id}/delete", post(enemies::delete_enemy))
        // Add CORS for development
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Editor listening on http://localhost:8080");

    axum::serve(listener, app).await?;

    Ok(())
}

/// Home page
async fn index() -> axum::response::Html<String> {
    axum::response::Html(html::page(
        "roguebench editor",
        r#"
        <h1>roguebench editor</h1>
        <nav>
            <ul>
                <li><a href="/enemies">Enemies</a></li>
            </ul>
        </nav>
        <p>Welcome to the roguebench content editor.</p>
        "#,
    ))
}
