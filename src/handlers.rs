use crate::{queries::*, structs::*, templates::*};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, Redirect},
};
use tower_sessions::Session;

pub mod oficinas;
pub mod usuarios;

const LOGIN_KEY: &str = "login";

pub async fn index(State(estado): State<AppState>, session: Session) -> Html<String> {
    let login = session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None });
    println!("{:?}", login);
    let html = match login.id {
        Some(id_integrante) => IndexTemplate {
            login: Some(get_nome(id_integrante, &estado.db).await),
        },
        None => IndexTemplate { login: None },
    };
    Html(html.render().unwrap())
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
        Some(id_integrante) => {
            if presente(id_integrante, id_oficina, &state.db).await {
                deleta_presenca(id_integrante, id_oficina, &state.db).await;
                Redirect::to(&format!("/oficinas/{id_oficina}"))
            } else {
                insere_presenca(id_integrante, id_oficina, &state.db).await;
                Redirect::to(&format!("/oficinas/{id_oficina}"))
            }
        }
        None => Redirect::to("/login"),
    }
}
