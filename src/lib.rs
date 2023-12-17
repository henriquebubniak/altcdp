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
pub struct OficinaPreview {
    pub titulo: String,
    pub id_oficina: i32,
    pub link_gravacao: String,
    pub nome_autor: String,
    pub data_oficina: String,
}

#[derive(Template)]
#[template(path = "oficinas.html")]
struct OficinasTemplate {
    oficinas: Vec<OficinaPreview>,
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
}

pub async fn oficinas_preview(State(state): State<AppState>) -> impl IntoResponse {
    let oficinas: Vec<OficinaPreview> = sqlx::query_as(
        r"select o.titulo, o.id_oficina, o.link_gravacao, i.nome nome_autor, o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante"
    )
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
    let oficinas: Vec<OficinaPreview> = sqlx::query_as(
        r"select o.titulo, o.id_oficina, o.link_gravacao, i.nome nome_autor, o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante
        and o.id_oficina = $1"
    )
        .bind(id)
        .fetch_all(&state.db)
        .await
        .unwrap();
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn index() -> impl IntoResponse {
    let html = IndexTemplate {};
    Html(html.render().unwrap())
}