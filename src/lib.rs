use crate::templates::{
    IndexTemplate, InscrevaSeTemplate, LoginTemplate, OficinaPreview, OficinasTemplate,
};
use askama::Template;
use axum::{
    debug_handler,
    extract::{Path, State},
    response::{Html, Redirect},
    Form,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use tower_sessions::Session;

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

pub async fn oficinas_preview(State(state): State<AppState>) -> Html<String> {
    let oficinas: Vec<OficinaPreview> = sqlx::query_as(
        r"
        select o.titulo, o.id_oficina, o.link_gravacao, i.nome nome_autor, o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante",
    )
    .fetch_all(&state.db)
    .await
    .unwrap();
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn oficina_detail(State(state): State<AppState>, Path(id): Path<i32>) -> Html<String> {
    let oficinas: Vec<OficinaPreview> = sqlx::query_as(
        r"
        select o.titulo, o.id_oficina, o.link_gravacao, i.nome nome_autor, o.data_oficina 
        from oficinas o, integrantes i
        where o.id_autor = i.id_integrante
        and o.id_oficina = $1",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .unwrap();
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn index(State(estado): State<AppState>, session: Session) -> Html<String> {
    let login = session.get::<Login>(LOGIN_KEY).await.unwrap();
    let html = match login {
        None => IndexTemplate { login: None },
        Some(l) => {
            match l.id {
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
                None => IndexTemplate { login: None }
            }
        }
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
            let login = Login {
                id: None,
            };
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
