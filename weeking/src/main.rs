

use std::convert::identity;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error,
    http::StatusCode,
    middleware, web, HttpMessage as _, HttpRequest, Responder,
};
use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
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

#[get("/login")]
async fn login(req: HttpRequest) -> impl Responder {
    Identity::login(&req.extensions(), "username_goes_here".to_owned()).unwrap();
    web::Redirect::to("/").using_status_code(StatusCode::ACCEPTED)
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
    let repo = Repository::init().await.expect("Db db db");
    dbg!(&repo);

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
        );
    };

    Ok(config.into())
}
