use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Usuario {
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
