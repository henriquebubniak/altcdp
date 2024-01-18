use crate::{
    handlers::LOGIN_KEY,
    queries::*,
    structs::{
        usuarios::{Credenciais, Usuario},
        *,
    },
    templates::*,
};
use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use tower_sessions::Session;

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
            println!("fail");
            let login = Login { id: None };
            session.insert(LOGIN_KEY, login).await.unwrap();
            Redirect::to("/login")
        }
        Some(id_integrante) => {
            println!("success");
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

pub async fn criar_usuario(
    State(state): State<AppState>,
    Form(criar_usuario): Form<Usuario>,
) -> Redirect {
    criar_usuario_db(criar_usuario, &state.db).await;
    Redirect::to("/")
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
        }
        None => Redirect::to("/login").into_response(),
    }
}
