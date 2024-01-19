use sqlx::{Pool, Postgres, Row};

use crate::structs::usuarios::{Credenciais, Usuario};

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

pub async fn criar_usuario_db(criar_usuario: Usuario, db: &Pool<Postgres>) {
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

pub async fn get_perfil(id_integrante: i32, db: &Pool<Postgres>) -> Usuario {
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
