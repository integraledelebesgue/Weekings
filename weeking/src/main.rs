use std::sync::Arc;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error,
    http::StatusCode,
    middleware, web, HttpMessage as _, HttpRequest, Responder,
};
use actix_web::{get, web::ServiceConfig};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use tokio::sync::Mutex;
use weeking::database::repository::Repository;

const FIVE_MINUTES: Duration = Duration::minutes(5);

#[get("/")]
async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String
}

#[get("/login")]
async fn login(form: web::Json<Login>, state: web::Data<Mutex<Repository>>, req: HttpRequest) -> impl Responder {
    let Login { username, password } = form.into_inner();

    match state.lock().await.check_user_data(&username, &password).await {
        Ok(_) => {
            Identity::login(&req.extensions(), username).unwrap();
            web::Redirect::to("/").using_status_code(StatusCode::ACCEPTED)
        },
        Err(_) => web::Redirect::to("/").using_status_code(StatusCode::UNAUTHORIZED)
    }
}

#[get("/feed")]
async fn feed() -> impl Responder {
    "Feed here"
}

#[get("/logout")]
async fn logout(id: Identity) -> impl Responder {
    id.logout();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let repo = Arc::new(Mutex::new(Repository::init().await.expect("Db db db")));

    let secret_key = Key::generate();

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .service(index)
                .service(login)
                .service(logout)
                .wrap(IdentityMiddleware::default())
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                        .cookie_name("auth-example".to_owned())
                        .cookie_secure(false)
                        .session_lifecycle(PersistentSession::default().session_ttl(FIVE_MINUTES))
                        .build(),
                )
                .wrap(middleware::NormalizePath::trim())
                .wrap(middleware::Logger::default()),
        )
            .app_data(repo.clone());
    };

    Ok(config.into())
}
