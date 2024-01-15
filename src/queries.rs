use sqlx::{types::chrono::NaiveDate, Pool, Postgres, Row};

use crate::structs::*;

pub async fn get_oficinas(db: &Pool<Postgres>) -> Vec<OficinaPreview> {
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
        nome_autor = nome_autor + &sobrenome;
        let data_oficina = row
            .get::<NaiveDate, &str>("data_oficina")
            .format("%d/%m/%Y")
            .to_string();
        let oficina_pre = OficinaPreview {
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

pub async fn get_oficina(db: &Pool<Postgres>, id_oficina: i32) -> OficinaPreview {
    let row = sqlx::query(
        r"
        select o.titulo, o.link_gravacao, i.nome, i.sobrenome , o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante
        and o.id_oficina = $1",
    )
    .bind(id_oficina)
    .fetch_one(db)
    .await
    .unwrap();
    let problemas: Vec<Problema> = sqlx::query_as(
        r"
        select p.link_problema, p.alias
        from problemas p
        where p.id_oficina = $1",
    )
    .bind(id_oficina)
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
    OficinaPreview {
        titulo: row.get("titulo"),
        id_oficina,
        link_gravacao: row.get("link_gravacao"),
        nome_autor,
        data_oficina,
        problemas,
    }
}

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
pub async fn get_nome(id_integrante: i32, db: &Pool<Postgres>) -> String {
    let u = sqlx::query(
        r"
        select nome 
        from integrantes i
        where i.id_integrante = $1",
    )
    .bind(id_integrante)
    .fetch_one(db)
    .await
    .unwrap();
    u.get("nome")
}

pub async fn verifica_credenciais(cred: Credenciais, db: &Pool<Postgres>) -> Option<i32> {
    println!("{:?}", cred);
    sqlx::query(
        r"
        select i.id_integrante 
        from integrantes i
        where i.email = $1
        and i.senha = $2",
    )
    .bind(cred.email)
    .bind(cred.senha)
    .fetch_optional(db)
    .await
    .unwrap()
    .map(|row| row.get("id_integrante"))
}

pub async fn criar_usuario_db(criar_usuario: CriarUsuario, db: &Pool<Postgres>) {
    let _ = sqlx::query(
        r"
        insert into integrantes (email, nome, sobrenome, senha)
        values ($1, $2, $3, $4)",
    )
    .bind(criar_usuario.email)
    .bind(criar_usuario.nome)
    .bind(criar_usuario.sobrenome)
    .bind(criar_usuario.senha)
    .execute(db)
    .await
    .unwrap();
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

pub async fn get_perfil(id_integrante: i32, db: &Pool<Postgres>) -> Perfil {
    sqlx::query_as(
        r"
        select i.email, i.nome, i.sobrenome, i.senha
        from integrantes i
        where i.id_integrante = $1",
    )
    .bind(id_integrante)
    .fetch_one(db)
    .await
    .unwrap()
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

pub async fn criar_oficina_db(criar_oficina: CriarOficina, db: &Pool<Postgres>) {
    sqlx::query(
        r"
        insert into oficinas (titulo, id_autor, data_oficina, link_gravacao)
        values ($1, $2, $3, $4)",
    )
    .bind(criar_oficina.titulo.clone())
    .bind(criar_oficina.id_autor)
    .bind(criar_oficina.data_oficina)
    .bind(criar_oficina.link_gravacao.clone())
    .execute(db)
    .await
    .unwrap();
    let id_oficina = sqlx::query(
        r"
        select id_oficina
        from oficinas
        where titulo = $1
        and id_autor = $2
        and data_oficina = $3
        and link_gravacao = $4",
    )
    .bind(criar_oficina.titulo)
    .bind(criar_oficina.id_autor)
    .bind(criar_oficina.data_oficina)
    .bind(criar_oficina.link_gravacao)
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
