use askama::Template;
use serde::{Deserialize, Serialize};

use crate::Problema;

#[derive(Template)]
#[template(path = "oficinas.html")]
pub struct OficinasTemplate {
    pub oficinas: Vec<OficinaPreview>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub login: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct OficinaPreview {
    pub titulo: String,
    pub id_oficina: i32,
    pub link_gravacao: String,
    pub nome_autor: String,
    pub data_oficina: String,
    pub problemas: Vec<Problema>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

#[derive(Template)]
#[template(path = "inscreva_se.html")]
pub struct InscrevaSeTemplate {}

#[derive(Template)]
#[template(path = "oficina.html")]
pub struct OficinaTemplate {
    pub oficina: OficinaPreview,
    pub login: bool,
    pub presente: bool,
}
