use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

pub mod oficinas;
pub mod usuarios;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Login {
    pub id: Option<i32>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Presenca {
    pub id_oficina: i32,
    pub titulo: String,
    pub data_oficina: String,
}
