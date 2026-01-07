//! Enemy editor routes
//!
//! CRUD operations for enemy definitions via HTML forms.

use super::html;
use crate::AppState;
use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    Form,
};
use roguebench_core::{EnemyDefinition, EnemyId};
use serde::Deserialize;

/// List all enemies
pub async fn list_enemies(State(state): State<AppState>) -> Html<String> {
    let enemies = {
        let db = state.db.lock().unwrap();
        db.get_all_enemies().unwrap_or_default()
    };

    let rows: String = enemies
        .iter()
        .map(|e| {
            format!(
                r#"<tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td class="actions">
                        <a href="/enemies/{}">View</a>
                        <a href="/enemies/{}/edit">Edit</a>
                        <form method="post" action="/enemies/{}/delete" style="display:inline">
                            <button type="submit" class="danger">Delete</button>
                        </form>
                    </td>
                </tr>"#,
                e.id.0, e.name, e.health, e.id.0, e.id.0, e.id.0
            )
        })
        .collect();

    let content = format!(
        r#"
        {}
        <h1>Enemies</h1>
        <p><a href="/enemies/new"><button>New Enemy</button></a></p>
        <table>
            <thead>
                <tr>
                    <th>ID</th>
                    <th>Name</th>
                    <th>Health</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
        "#,
        html::nav(),
        if rows.is_empty() {
            "<tr><td colspan=\"4\">No enemies defined yet.</td></tr>".to_string()
        } else {
            rows
        }
    );

    Html(html::page("Enemies - roguebench", &content))
}

/// Show a single enemy
pub async fn show_enemy(State(state): State<AppState>, Path(id): Path<String>) -> Html<String> {
    let enemy_id = EnemyId::new(&id);
    let result = {
        let db = state.db.lock().unwrap();
        db.get_enemy(&enemy_id)
    };

    match result {
        Ok(enemy) => {
            let content = format!(
                r#"
                {}
                <h1>{}</h1>
                <dl>
                    <dt>ID</dt>
                    <dd>{}</dd>
                    <dt>Health</dt>
                    <dd>{}</dd>
                </dl>
                <p>
                    <a href="/enemies/{}/edit"><button>Edit</button></a>
                    <a href="/enemies"><button>Back to List</button></a>
                </p>
                "#,
                html::nav(),
                enemy.name,
                enemy.id.0,
                enemy.health,
                enemy.id.0
            );
            Html(html::page(&format!("{} - roguebench", enemy.name), &content))
        }
        Err(_) => {
            let content = format!(
                r#"
                {}
                <h1>Enemy Not Found</h1>
                <p>No enemy with ID "{}" exists.</p>
                <a href="/enemies"><button>Back to List</button></a>
                "#,
                html::nav(),
                id
            );
            Html(html::page("Not Found - roguebench", &content))
        }
    }
}

/// Form data for creating/updating enemies
#[derive(Deserialize)]
pub struct EnemyForm {
    pub id: String,
    pub name: String,
    pub health: i32,
}

/// New enemy form
pub async fn new_enemy_form() -> Html<String> {
    let content = format!(
        r#"
        {}
        <h1>New Enemy</h1>
        <form method="post" action="/enemies">
            <label>
                ID (unique identifier)
                <input type="text" name="id" required placeholder="e.g., grunt">
            </label>
            <label>
                Name
                <input type="text" name="name" required placeholder="e.g., Grunt">
            </label>
            <label>
                Health
                <input type="number" name="health" required value="100" min="1">
            </label>
            <button type="submit">Create Enemy</button>
        </form>
        <p><a href="/enemies">Cancel</a></p>
        "#,
        html::nav()
    );

    Html(html::page("New Enemy - roguebench", &content))
}

/// Create a new enemy
pub async fn create_enemy(State(state): State<AppState>, Form(form): Form<EnemyForm>) -> Redirect {
    let enemy = EnemyDefinition::new(&form.id, &form.name, form.health);

    let result = {
        let db = state.db.lock().unwrap();
        db.upsert_enemy(&enemy)
    };

    if let Err(e) = result {
        tracing::error!("Failed to create enemy: {}", e);
    } else {
        tracing::info!("Created enemy: {} ({})", enemy.name, enemy.id.0);
    }

    Redirect::to("/enemies")
}

/// Edit enemy form
pub async fn edit_enemy_form(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Html<String> {
    let enemy_id = EnemyId::new(&id);
    let result = {
        let db = state.db.lock().unwrap();
        db.get_enemy(&enemy_id)
    };

    match result {
        Ok(enemy) => {
            let content = format!(
                r#"
                {}
                <h1>Edit Enemy: {}</h1>
                <form method="post" action="/enemies/{}">
                    <label>
                        ID (read-only)
                        <input type="text" name="id" value="{}" readonly>
                    </label>
                    <label>
                        Name
                        <input type="text" name="name" required value="{}">
                    </label>
                    <label>
                        Health
                        <input type="number" name="health" required value="{}" min="1">
                    </label>
                    <button type="submit">Save Changes</button>
                </form>
                <p><a href="/enemies">Cancel</a></p>
                "#,
                html::nav(),
                enemy.name,
                enemy.id.0,
                enemy.id.0,
                enemy.name,
                enemy.health
            );
            Html(html::page(
                &format!("Edit {} - roguebench", enemy.name),
                &content,
            ))
        }
        Err(_) => {
            let content = format!(
                r#"
                {}
                <h1>Enemy Not Found</h1>
                <p>No enemy with ID "{}" exists.</p>
                <a href="/enemies"><button>Back to List</button></a>
                "#,
                html::nav(),
                id
            );
            Html(html::page("Not Found - roguebench", &content))
        }
    }
}

/// Update an enemy
pub async fn update_enemy(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Form(form): Form<EnemyForm>,
) -> Redirect {
    // Use the path ID, not the form ID (form ID is read-only)
    let enemy = EnemyDefinition::new(&id, &form.name, form.health);

    let result = {
        let db = state.db.lock().unwrap();
        db.upsert_enemy(&enemy)
    };

    if let Err(e) = result {
        tracing::error!("Failed to update enemy: {}", e);
    } else {
        tracing::info!("Updated enemy: {} ({})", enemy.name, enemy.id.0);
    }

    Redirect::to("/enemies")
}

/// Delete an enemy
pub async fn delete_enemy(State(state): State<AppState>, Path(id): Path<String>) -> Redirect {
    let enemy_id = EnemyId::new(&id);

    let result = {
        let db = state.db.lock().unwrap();
        db.delete_enemy(&enemy_id)
    };

    if let Err(e) = result {
        tracing::error!("Failed to delete enemy: {}", e);
    } else {
        tracing::info!("Deleted enemy: {}", id);
    }

    Redirect::to("/enemies")
}
