use askama::Template;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Participant {
    pub participant_id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Oficina {
    pub title: String,
    pub workshop_id: i32,
    pub recording_link: String,
    pub author_id: i32,
    pub _date: String,
}

#[derive(Template)]
#[template(path = "oficinas.html")]
struct OficinasTemplate {
    oficinas: Vec<Oficina>,
}

pub async fn oficinas(State(state): State<AppState>) -> impl IntoResponse {
    let oficinas: Vec<Oficina> = sqlx::query_as("select * from workshops")
        .fetch_all(&state.db)
        .await
        .unwrap();
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn oficina_detail<'a>(
    State(state): State<AppState>,
    Path(id): Path<i32>,   
) -> impl IntoResponse {
    let oficinas: Vec<Oficina> = sqlx::query_as("select * from workshops where workshop_id = $1")
        .bind(id)
        .fetch_all(&state.db)
        .await
        .unwrap();
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}
