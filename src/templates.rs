use askama::Template;

use crate::structs::{oficinas::Oficina, usuarios::Usuario, Presenca};

#[derive(Template)]
#[template(path = "oficinas.html")]
pub struct OficinasTemplate {
    pub oficinas: Vec<Oficina>,
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
    pub oficina: &'a Oficina,
    pub login: bool,
    pub presente: bool,
}

#[derive(Template)]
#[template(path = "perfil.html")]
pub struct PerfilTemplate {
    pub perfil: Option<Usuario>,
    pub presencas: Vec<Presenca>,
}

#[derive(Template)]
#[template(path = "criar_oficina.html")]
pub struct CriarOficinaTemplate {}
