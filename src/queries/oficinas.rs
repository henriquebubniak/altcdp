use sqlx::{types::chrono::NaiveDate, Pool, Postgres, Row};

use crate::structs::oficinas::{CriarOficina, Oficina, Problema};

pub async fn get_oficinas(db: &Pool<Postgres>, id_oficina: Option<i32>) -> Vec<Oficina> {
    let mut oficinas = Vec::new();
    let mut query = r"
        select o.titulo, o.id_oficina, o.link_gravacao, i.nome, i.sobrenome , o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante"
        .to_owned();
    if let Some(id_oficina) = id_oficina {
        query = query + &format!("\n and o.id_oficina = {}", id_oficina);
    }
    let result = sqlx::query(&query).fetch_all(db).await.unwrap();
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
        nome_autor = nome_autor + &sobrenome;
        let data_oficina = row
            .get::<NaiveDate, &str>("data_oficina")
            .format("%d/%m/%Y")
            .to_string();
        let oficina_pre = Oficina {
            titulo: row.get("titulo"),
            id_oficina: row.get("id_oficina"),
            link_gravacao: row.get("link_gravacao"),
            nome_autor,
            data_oficina,
            problemas,
        };
        oficinas.push(oficina_pre);
    }
    oficinas
}

pub async fn criar_oficina_db(criar_oficina: CriarOficina, db: &Pool<Postgres>) {
    let id_oficina = sqlx::query(
        r"
        insert into oficinas (titulo, id_autor, data_oficina, link_gravacao)
        values ($1, $2, $3, $4)
        returning id_oficina",
    )
    .bind(criar_oficina.titulo.clone())
    .bind(criar_oficina.id_autor)
    .bind(criar_oficina.data_oficina)
    .bind(criar_oficina.link_gravacao.clone())
    .fetch_one(db)
    .await
    .unwrap();
    let id_oficina: i32 = id_oficina.get("id_oficina");
    for problema in criar_oficina.problemas {
        sqlx::query(
            r"
                insert into problemas (id_oficina, alias, link_problema)
                values ($1, $2, $3)",
        )
        .bind(id_oficina)
        .bind(problema.alias)
        .bind(problema.link_problema)
        .execute(db)
        .await
        .unwrap();
    }
}
