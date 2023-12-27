use crate::queries::{
    criar_usuario_db, deleta_presenca, get_nome, get_oficinas, insere_presenca, presente,
    verifica_credenciais,
};
use askama::Template;
use axum::{
    debug_handler,
    extract::{Path, State},
    response::{Html, Redirect},
    Form,
};
pub use structs::{AppState, Credenciais, CriarUsuario, Login};
use templates::{
    IndexTemplate, InscrevaSeTemplate, LoginTemplate, OficinaTemplate, OficinasTemplate,
};
use tower_sessions::Session;

mod queries;
mod structs;
mod templates;

const LOGIN_KEY: &str = "login";

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
    let oficinas = get_oficinas(&state.db).await;
    let html = match session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None })
    {
        Login {
            id: Some(id_integrante),
        } => OficinaTemplate {
            oficina: &oficinas[0],
            login: true,
            presente: presente(id_integrante, id_oficina, &state.db).await,
        },
        Login { id: None } => OficinaTemplate {
            oficina: &oficinas[0],
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
        Some(id_integrante) => IndexTemplate {
            login: Some(get_nome(id_integrante, &estado.db).await),
        },
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
    Form(cred): Form<Credenciais>,
) -> Redirect {
    match verifica_credenciais(cred, &estado.db).await {
        None => {
            let login = Login { id: None };
            session.insert(LOGIN_KEY, login).await.unwrap();
            Redirect::to("/login")
        }
        Some(id_integrante) => {
            let login = Login {
                id: Some(id_integrante),
            };
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
    criar_usuario_db(criar_usuario, &state.db).await;
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
