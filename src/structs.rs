use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres, types::chrono::NaiveDate};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct OficinaPreview {
    pub titulo: String,
    pub id_oficina: i32,
    pub link_gravacao: String,
    pub nome_autor: String,
    pub data_oficina: String,
    pub problemas: Vec<Problema>,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Login {
    pub id: Option<i32>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Deserialize, Serialize)]
pub struct CriarUsuario {
    pub email: String,
    pub nome: String,
    pub sobrenome: String,
    pub senha: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Credenciais {
    pub email: String,
    pub senha: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Problema {
    pub link_problema: String,
    pub alias: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Perfil {
    pub email: String,
    pub nome: String,
    pub sobrenome: String,
    pub senha: String,
}
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Presenca {
    pub id_oficina: i32,
    pub titulo: String,
    pub data_oficina: String,
}

pub struct CriarOficina {
    pub titulo: String,
	pub link_gravacao: Option<String>,
	pub id_autor: i32,
	pub data_oficina: NaiveDate,
}

#[derive(Deserialize, Serialize)]
pub struct CriarOficinaForm {
    pub titulo: String,
	pub link_gravacao: Option<String>,
}
