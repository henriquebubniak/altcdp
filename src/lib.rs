use crate::templates::{
    IndexTemplate, InscrevaSeTemplate, LoginTemplate, OficinaPreview, OficinaTemplate,
    OficinasTemplate,
};
use askama::Template;
use axum::{
    debug_handler,
    extract::{Path, State},
    response::{Html, Redirect},
    Form,
};
use queries::{get_oficina_preview_one, get_oficina_preview_vec};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres, Row};
use tower_sessions::Session;

mod queries;

mod templates;

const LOGIN_KEY: &str = "login";

#[derive(Default, Deserialize, Serialize, Debug)]
struct Login {
    id: Option<i32>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Deserialize, Serialize)]
pub struct CriarUsuario {
    email: String,
    nome: String,
    sobrenome: String,
    senha: String,
}

#[derive(Deserialize, Serialize)]
pub struct Credenciais {
    email: String,
    senha: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Problema{
    link_problema: String,
    alias: String,
}

pub async fn oficinas_preview(State(state): State<AppState>) -> Html<String> {
    let oficinas = get_oficina_preview_vec(&state.db).await;
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn oficina_detail(
    State(state): State<AppState>,
    session: Session,
    Path(id_oficina): Path<i32>,
) -> Html<String> {
    let oficina = get_oficina_preview_one(&state.db).await;
    let html = match session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None })
    {
        Login {
            id: Some(id_integrante),
        } => {
            let presenca = sqlx::query(
                r"
                select *
                from presenca p
                where p.id_integrante = $1
                and p.id_oficina = $2",
            )
            .bind(id_integrante)
            .bind(id_oficina)
            .fetch_all(&state.db)
            .await
            .unwrap();
            match presenca.len() {
                0 => OficinaTemplate {
                    oficina,
                    login: true,
                    presente: false,
                },
                _ => OficinaTemplate {
                    oficina,
                    login: true,
                    presente: true,
                },
            }
        }
        Login { id: None } => OficinaTemplate {
            oficina,
            login: false,
            presente: false,
        },
    };
    Html(html.render().unwrap())
}

pub async fn index(State(estado): State<AppState>, session: Session) -> Html<String> {
    let login = session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None });
    let html = match login.id {
        Some(id) => {
            let u = sqlx::query(
                r"
                select nome 
                from integrantes i
                where i.id_integrante = $1",
            )
            .bind(id)
            .fetch_all(&estado.db)
            .await
            .unwrap();
            let u: String = u[0].get("nome");
            IndexTemplate { login: Some(u) }
        }
        None => IndexTemplate { login: None },
    };
    Html(html.render().unwrap())
}

pub async fn login() -> Html<String> {
    let html = LoginTemplate {};
    Html(html.render().unwrap())
}

pub async fn verifica_login(
    State(estado): State<AppState>,
    session: Session,
    Form(credenciais): Form<Credenciais>,
) -> Redirect {
    let user = sqlx::query(
        r"select * 
        from integrantes 
        where email = $1
        and senha = $2",
    )
    .bind(credenciais.email)
    .bind(credenciais.senha)
    .fetch_all(&estado.db)
    .await
    .unwrap();
    match user.len() {
        0 => {
            let login = Login { id: None };
            session.insert(LOGIN_KEY, login).await.unwrap();
            Redirect::to("/login")
        }
        _ => {
            let login = Login {
                id: Some(user[0].get("id_integrante")),
            };
            println!("{:?}", login);
            session.insert(LOGIN_KEY, login).await.unwrap();
            Redirect::to("/")
        }
    }
}

pub async fn logout(session: Session) -> Redirect {
    session.insert(LOGIN_KEY, Login { id: None }).await.unwrap();
    Redirect::to("/")
}

pub async fn inscreva_se() -> Html<String> {
    let html = InscrevaSeTemplate {};
    Html(html.render().unwrap())
}

#[debug_handler]
pub async fn criar_usuario(
    State(state): State<AppState>,
    Form(criar_usuario): Form<CriarUsuario>,
) -> Redirect {
    let _ = sqlx::query(
        r"
        insert into integrantes (email, nome, sobrenome, senha)
        values ($1, $2, $3, $4)",
    )
    .bind(criar_usuario.email)
    .bind(criar_usuario.nome)
    .bind(criar_usuario.sobrenome)
    .bind(criar_usuario.senha)
    .execute(&state.db)
    .await
    .unwrap();
    Redirect::to("/")
}

pub async fn presenca(
    State(state): State<AppState>,
    session: Session,
    Path(id_oficina): Path<i32>,
) -> Redirect {
    let login = session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None });
    match login.id {
        Some(id_usuario) => {
            let presenca = sqlx::query(
                r"
                select *
                from presenca p
                where p.id_integrante = $1
                and p.id_oficina = $2",
            )
            .bind(id_usuario)
            .bind(id_oficina)
            .fetch_all(&state.db)
            .await
            .unwrap();
            match presenca.len() {
                0 => {
                    let _ = sqlx::query(
                        r"
                        insert into presenca (id_integrante, id_oficina)
                        values ($1, $2)",
                    )
                    .bind(id_usuario)
                    .bind(id_oficina)
                    .execute(&state.db)
                    .await
                    .unwrap();
                    Redirect::to(&format!("/oficinas/{id_oficina}"))
                }
                _ => {
                    let _ = sqlx::query(
                        r"
                        delete from presenca p
                        where p.id_integrante = $1
                        and p.id_oficina = $2",
                    )
                    .bind(id_usuario)
                    .bind(id_oficina)
                    .execute(&state.db)
                    .await
                    .unwrap();
                    Redirect::to(&format!("/oficinas/{id_oficina}"))
                }
            }
        }
        None => Redirect::to("/login"),
    }
}
