use crate::{
    handlers::LOGIN_KEY,
    queries::*,
    structs::{
        oficinas::{CriarOficina, CriarOficinaForm, Problema},
        *,
    },
    templates::*,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use sqlx::types::chrono::Utc;
use tower_sessions::Session;

pub async fn oficinas_preview(State(state): State<AppState>) -> Html<String> {
    let oficinas = get_oficinas(&state.db).await;
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn oficina_detail(
    State(state): State<AppState>,
    session: Session,
    Path(id_oficina): Path<i32>,
) -> Html<String> {
    let oficina = get_oficina(&state.db, id_oficina).await;
    let html = match session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None })
    {
        Login {
            id: Some(id_integrante),
        } => OficinaTemplate {
            oficina: &oficina,
            login: true,
            presente: presente(id_integrante, id_oficina, &state.db).await,
        },
        Login { id: None } => OficinaTemplate {
            oficina: &oficina,
            login: false,
            presente: false,
        },
    };
    Html(html.render().unwrap())
}

pub async fn criar_oficina(session: Session) -> impl IntoResponse {
    let login = session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None });
    match login.id {
        Some(_) => {
            let html = CriarOficinaTemplate {};
            Html(html.render().unwrap()).into_response()
        }
        None => Redirect::to("/login").into_response(),
    }
}

pub async fn criar_oficina_form(
    session: Session,
    State(state): State<AppState>,
    Form(criar_oficina_form): Form<CriarOficinaForm>,
) -> Redirect {
    println!("{:?}", criar_oficina_form);
    let login = session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None });
    match login.id {
        Some(id_autor) => {
            let problemas_links = match criar_oficina_form.problemas_links {
                Some(links) => links.split(';').map(|link| link.to_owned()).collect(),
                None => vec![],
            };
            let problemas_alias = match criar_oficina_form.problemas_alias {
                Some(links) => links.split(';').map(|link| link.to_owned()).collect(),
                None => vec![],
            };
            if problemas_links.len() != problemas_alias.len() {
                return Redirect::to("/criar_oficina");
            }
            let mut problemas = Vec::new();
            for i in 0..problemas_links.len() {
                problemas.push(Problema {
                    alias: problemas_alias[i].to_owned(),
                    link_problema: problemas_links[i].to_owned(),
                })
            }
            let criar_oficina = CriarOficina {
                titulo: criar_oficina_form.titulo,
                link_gravacao: criar_oficina_form.link_gravacao,
                id_autor,
                data_oficina: Utc::now().date_naive(),
                problemas,
            };
            criar_oficina_db(criar_oficina, &state.db).await;
            Redirect::to("/oficinas")
        }
        None => Redirect::to("/login"),
    }
}
