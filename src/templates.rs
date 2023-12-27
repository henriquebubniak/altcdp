use askama::Template;

use crate::structs::OficinaPreview;

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

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

#[derive(Template)]
#[template(path = "inscreva_se.html")]
pub struct InscrevaSeTemplate {}

#[derive(Template)]
#[template(path = "oficina.html")]
pub struct OficinaTemplate<'a> {
    pub oficina: &'a OficinaPreview,
    pub login: bool,
    pub presente: bool,
}
