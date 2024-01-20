use crate::{
    handlers::LOGIN_KEY,
    queries::{oficinas::*, presente},
    structs::{
        oficinas::{CriarOficina, Problema},
        AppState, Login,
    },
    templates::*,
};
use askama::Template;
use axum::{
    extract::{Multipart, Path, State},
    response::{Html, IntoResponse, Redirect},
};
use sqlx::types::chrono::Utc;
use tower_sessions::Session;

pub async fn oficinas_preview(State(state): State<AppState>) -> Html<String> {
    let oficinas = get_oficinas(&state.db, None).await;
    let html = OficinasTemplate { oficinas };
    Html(html.render().unwrap())
}

pub async fn oficina_detail(
    State(state): State<AppState>,
    session: Session,
    Path(id_oficina): Path<i32>,
) -> Html<String> {
    let oficina = get_oficinas(&state.db, Some(id_oficina)).await;
    let html = match session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None })
    {
        Login {
            id: Some(id_integrante),
        } => OficinaTemplate {
            oficina: &oficina[0],
            login: true,
            presente: presente(id_integrante, id_oficina, &state.db).await,
        },
        Login { id: None } => OficinaTemplate {
            oficina: &oficina[0],
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
    mut form: Multipart,
) -> Redirect {
    let login = session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None });
    match login.id {
        Some(id_autor) => {
            let mut problemas_links = Vec::new();
            let mut problemas_alias = Vec::new();
            let mut criar_oficina = CriarOficina {
                titulo: "".to_owned(),
                link_gravacao: None,
                id_autor,
                data_oficina: Utc::now().date_naive(),
                problemas: vec![],
                descricao: "".to_owned(),
            };
            while let Some(field) = form.next_field().await.unwrap() {
                let name = field.name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                match name.as_str() {
                    "titulo" => criar_oficina.titulo = String::from_utf8(data.to_vec()).unwrap(),
                    "link_gravacao" => {
                        criar_oficina.link_gravacao =
                            Some(String::from_utf8(data.to_vec()).unwrap())
                    }
                    "problemas_links" => {
                        problemas_links = String::from_utf8(data.to_vec())
                            .unwrap()
                            .split(';')
                            .map(|link| link.to_owned())
                            .collect()
                    }
                    "problemas_alias" => {
                        problemas_alias = String::from_utf8(data.to_vec())
                            .unwrap()
                            .split(';')
                            .map(|link| link.to_owned())
                            .collect()
                    }
                    "descricao" => {
                        let markdown_input = String::from_utf8(data.to_vec()).unwrap();
                        let parser = pulldown_cmark::Parser::new(&markdown_input);

                        // Write to a new String buffer.
                        let mut html_output = String::new();
                        pulldown_cmark::html::push_html(&mut html_output, parser);
                        criar_oficina.descricao = html_output;
                    }

                    _ => (),

                }
            }
            println!("\n\n\n{:?},\n{:?}\n\n\n", problemas_alias, problemas_links);
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
            criar_oficina.problemas = problemas;
            criar_oficina_db(criar_oficina, &state.db).await;
            Redirect::to("/oficinas")
        }
        None => Redirect::to("/login"),
    }
}
