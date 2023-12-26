use crate::templates::{
    IndexTemplate, InscrevaSeTemplate, LoginTemplate, OficinaPreview, OficinasTemplate,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
    Form,
    debug_handler
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

mod templates;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Deserialize, Serialize)]
pub struct CriarUsuario {
    email: String,
    nome: String,
    sobrenome: String,
    senha: String,
}

pub async fn oficinas_preview(State(state): State<AppState>) -> Html<String> {
    let oficinas: Vec<OficinaPreview> = sqlx::query_as(
        r"
        select o.titulo, o.id_oficina, o.link_gravacao, i.nome nome_autor, o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante",
    )
    .fetch_all(&state.db)
    .await
    .unwrap();
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn oficina_detail(State(state): State<AppState>, Path(id): Path<i32>) -> Html<String> {
    let oficinas: Vec<OficinaPreview> = sqlx::query_as(
        r"
        select o.titulo, o.id_oficina, o.link_gravacao, i.nome nome_autor, o.data_oficina 
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

pub async fn index() -> Html<String> {
    let html = IndexTemplate { login: false };
    Html(html.render().unwrap())
}

pub async fn login() -> Html<String> {
    let html = LoginTemplate {};
    Html(html.render().unwrap())
}

pub async fn inscreva_se() -> Html<String> {
    let html = InscrevaSeTemplate {};
    Html(html.render().unwrap())
}

#[debug_handler]
pub async fn criar_usuario(
    State(state): State<AppState>,
    Form(criar_usuario): Form<CriarUsuario>,
) -> Redirect {
    let _ = sqlx::query(
        r"
        insert into integrantes (email, nome, sobrenome, senha)
        values ($1, $2, $3, $4)"
    )
    .bind(criar_usuario.email)
    .bind(criar_usuario.nome)
    .bind(criar_usuario.sobrenome)
    .bind(criar_usuario.senha)
    .execute(&state.db)
    .await
    .unwrap();
    Redirect::to("/")
}
