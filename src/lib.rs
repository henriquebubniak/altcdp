use crate::templates::{IndexTemplate, InscrevaSeTemplate, OficinaPreview, OficinasTemplate};
use askama::Template;
use axum::extract::{Path, State};
use axum::response::{Html, Redirect};
use axum::Form;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::sync::{Arc, Mutex};

mod templates;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub login: Arc<Mutex<bool>>,
}

#[derive(Deserialize, Debug)]
pub struct Usuario {
    username: String,
    password: String,
}

pub async fn oficinas_preview(State(state): State<AppState>) -> Html<String> {
    let oficinas: Vec<OficinaPreview> = sqlx::query_as(
        r"select o.titulo, o.id_oficina, o.link_gravacao, i.nome nome_autor, o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante",
    )
    .fetch_all(&state.db)
    .await
    .unwrap();
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn verifica_login(State(state): State<AppState>, Form(body): Form<Usuario>) -> Redirect {
    let user = sqlx::query(
        r"select * 
        from integrantes 
        where email = $1
        and senha = $2",
    )
    .bind(body.username)
    .bind(body.password)
    .fetch_all(&state.db)
    .await
    .unwrap();
    match user.len() {
        0 => Redirect::to("/inscreva_se"),
        _ => {
            *state.login.lock().unwrap() = true;
            Redirect::to("/")
        }
    }
}

pub async fn logout(State(state): State<AppState>) -> Redirect {
    *state.login.lock().unwrap() = false;
    Redirect::to("/")
}

pub async fn oficina_detail(State(state): State<AppState>, Path(id): Path<i32>) -> Html<String> {
    let oficinas: Vec<OficinaPreview> = sqlx::query_as(
        r"select o.titulo, o.id_oficina, o.link_gravacao, i.nome nome_autor, o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante
        and o.id_oficina = $1",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .unwrap();
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn index(State(state): State<AppState>) -> Html<String> {
    let html = IndexTemplate {
        login: *state.login.lock().unwrap(),
    };
    Html(html.render().unwrap())
}

pub async fn inscreva_se() -> Html<String> {
    let html = InscrevaSeTemplate {};
    Html(html.render().unwrap())
}
