use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono::NaiveDate};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Oficina {
    pub titulo: String,
    pub id_oficina: i32,
    pub link_gravacao: String,
    pub nome_autor: String,
    pub data_oficina: String,
    pub problemas: Vec<Problema>,
}

pub struct CriarOficina {
    pub titulo: String,
    pub link_gravacao: Option<String>,
    pub id_autor: i32,
    pub data_oficina: NaiveDate,
    pub problemas: Vec<Problema>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CriarOficinaForm {
    pub titulo: String,
    pub link_gravacao: Option<String>,
    pub problemas_alias: Option<String>,
    pub problemas_links: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Problema {
    pub link_problema: String,
    pub alias: String,
}
