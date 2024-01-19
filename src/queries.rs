use sqlx::{types::chrono::NaiveDate, Pool, Postgres, Row};

use crate::structs::*;

pub mod oficinas;
pub mod usuarios;

pub async fn presente(id_integrante: i32, id_oficina: i32, db: &Pool<Postgres>) -> bool {
    sqlx::query(
        r"
    select *
    from presenca p
    where p.id_integrante = $1
    and p.id_oficina = $2",
    )
    .bind(id_integrante)
    .bind(id_oficina)
    .fetch_optional(db)
    .await
    .unwrap()
    .is_some()
}

pub async fn insere_presenca(id_integrante: i32, id_oficina: i32, db: &Pool<Postgres>) {
    let _ = sqlx::query(
        r"
        insert into presenca (id_integrante, id_oficina)
        values ($1, $2)",
    )
    .bind(id_integrante)
    .bind(id_oficina)
    .execute(db)
    .await
    .unwrap();
}

pub async fn deleta_presenca(id_integrante: i32, id_oficina: i32, db: &Pool<Postgres>) {
    let _ = sqlx::query(
        r"
        delete from presenca p
        where p.id_integrante = $1
        and p.id_oficina = $2",
    )
    .bind(id_integrante)
    .bind(id_oficina)
    .execute(db)
    .await
    .unwrap();
}

pub async fn get_presencas(id_integrante: i32, db: &Pool<Postgres>) -> Vec<Presenca> {
    let mut presencas = Vec::new();
    let result = sqlx::query(
        r"
        select o.titulo, o.data_oficina, o.id_oficina
        from oficinas o, presenca p
        where p.id_integrante = $1
        and p.id_oficina = o.id_oficina",
    )
    .bind(id_integrante)
    .fetch_all(db)
    .await
    .unwrap();
    for row in result {
        let data_oficina = row
            .get::<NaiveDate, &str>("data_oficina")
            .format("%d/%m/%Y")
            .to_string();
        presencas.push(Presenca {
            data_oficina,
            titulo: row.get("titulo"),
            id_oficina: row.get("id_oficina"),
        });
    }
    presencas
}
