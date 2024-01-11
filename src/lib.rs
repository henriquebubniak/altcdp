use crate::queries::{
    criar_usuario_db, deleta_presenca, get_nome, get_oficinas, get_perfil, get_presencas,
    insere_presenca, presente, verifica_credenciais, criar_oficina_db
};
use askama::Template;
use axum::{
    debug_handler,
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use sqlx::types::chrono::Utc;
pub use structs::{AppState, Credenciais, CriarUsuario, Login, CriarOficinaForm, CriarOficina};
use templates::{
    IndexTemplate, InscrevaSeTemplate, LoginTemplate, OficinaTemplate, OficinasTemplate,
    PerfilTemplate, CriarOficinaTemplate,
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

#[debug_handler]
pub async fn criar_oficina_form(
    session: Session,
    State(state): State<AppState>,
    Form(criar_oficina_form): Form<CriarOficinaForm>,
) -> Redirect {
    let login = session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None });
    match login.id {
        Some(id_autor) => {
            let criar_oficina = CriarOficina {
                titulo: criar_oficina_form.titulo,
                link_gravacao: criar_oficina_form.link_gravacao,
                id_autor,
                data_oficina: Utc::now().date_naive(),
            };
            criar_oficina_db(criar_oficina, &state.db).await;
            Redirect::to("/oficinas")
        }
        None => Redirect::to("/login")
    }
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

pub async fn perfil(State(estado): State<AppState>, session: Session) -> Response {
    let login = session
        .get::<Login>(LOGIN_KEY)
        .await
        .unwrap()
        .unwrap_or(Login { id: None });
    match login.id {
        Some(id_integrante) => {
            let html = PerfilTemplate {
                perfil: Some(get_perfil(id_integrante, &estado.db).await),
                presencas: get_presencas(id_integrante, &estado.db).await,
            };
            Html(html.render().unwrap()).into_response()
        },
        None => Redirect::to("/login").into_response()
    }
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
        None => Redirect::to("/login").into_response()
    }
}
