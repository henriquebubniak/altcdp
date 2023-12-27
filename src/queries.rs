use sqlx::{Pool, Postgres, Row};

use crate::{Problema, OficinaPreview};

pub async fn get_oficina_preview_vec(db: &Pool<Postgres>) -> Vec<OficinaPreview> {
    let mut oficinas = Vec::new();
    let result = sqlx::query(
        r"
        select o.titulo, o.id_oficina, o.link_gravacao, i.nome, i.sobrenome , o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante",
    )
    .fetch_all(db)
    .await
    .unwrap();
    for row in result {
        let problemas: Vec<Problema> = sqlx::query_as(
            r"
            select p.link_problema, p.alias
            from problemas p
            where p.id_oficina = $1",
        )
        .bind(row.get::<i32, &str>("id_oficina"))
        .fetch_all(db)
        .await
        .unwrap();
        let sobrenome: String = row.get("sobrenome");
        let mut nome_autor: String = row.get("nome");
        nome_autor.push(' ');
        nome_autor.push_str(&sobrenome);
        let oficina_pre = OficinaPreview {
            titulo: row.get("titulo"),
            id_oficina: row.get("id_oficina"),
            link_gravacao: row.get("link_gravacao"),
            nome_autor,
            data_oficina: row.get("data_oficina"),
            problemas,
        };
        oficinas.push(oficina_pre);
    }
    oficinas
}

pub async fn get_oficina_preview_one(db: &Pool<Postgres>) -> OficinaPreview {
    let row = sqlx::query(
        r"
        select o.titulo, o.id_oficina, o.link_gravacao, i.nome, i.sobrenome , o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante",
    )
    .fetch_one(db)
    .await
    .unwrap();
    let problemas: Vec<Problema> = sqlx::query_as(
        r"
        select p.link_problema, p.alias
        from problemas p
        where p.id_oficina = $1",
    )
    .bind(row.get::<i32, &str>("id_oficina"))
    .fetch_all(db)
    .await
    .unwrap();
    let sobrenome: String = row.get("sobrenome");
    let mut nome_autor: String = row.get("nome");
    nome_autor.push(' ');
    nome_autor.push_str(&sobrenome);
    OficinaPreview {
        titulo: row.get("titulo"),
        id_oficina: row.get("id_oficina"),
        link_gravacao: row.get("link_gravacao"),
        nome_autor,
        data_oficina: row.get("data_oficina"),
        problemas,
    }
}
