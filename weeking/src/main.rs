use std::convert::identity;
use std::io;
use std::sync::Arc;
use actix_cors::Cors;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::{time::Duration, Key}, error, http::StatusCode, middleware, web, HttpMessage as _, HttpRequest, Responder, post};
use actix_web::{get, web::ServiceConfig};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use tokio::sync::Mutex;
use weeking::database::repository::Repository;
use weeking::routes;
use weeking::state::State;
use derive_error::Error;

const FIVE_MINUTES: Duration = Duration::minutes(5);

#[derive(Debug, Error, Serialize, Deserialize)]
enum Error {
    UnauthorizedUser,
    InternalServerError
}

#[get("/")]
async fn index(identity: Option<Identity>) -> actix_web::Result<impl Responder> {
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    Ok(format!("Hello {id}"))
}

#[derive(Serialize, Deserialize)]
struct Login {
    username: String,
    password: String
}

#[post("/login")]
async fn login(payload: web::Json<Login>, req: HttpRequest, state: web::Data<Mutex<State>>) -> impl Responder {
    let Login { username, password } = payload.into_inner();

    let validation = state
        .lock()
        .await
        .repository
        .check_user_data(&username, &password)
        .await;

    match validation {
        Ok(None) | Err(_) => {
            web::Redirect::to("/").using_status_code(StatusCode::UNAUTHORIZED)
        },
        Ok(_) => {
            Identity::login(&req.extensions(), username).unwrap();
            web::Redirect::to("/").using_status_code(StatusCode::ACCEPTED)
        }
    }
}

#[get("/feed")]
async fn feed(identity: Option<Identity>, state: web::Data<Mutex<State>>) -> impl Responder {
    if identity.is_none() {
        return web::Json(Err(Error::InternalServerError));
    }

    let session_id = identity
        .unwrap()
        .id()
        .unwrap();

    let id = *state
        .lock()
        .await
        .users.get(&session_id)
        .unwrap();

    web::Json(Ok(()))
}

#[get("/groups")]
async fn all_groups(identity: Option<Identity>, state: web::Data<Mutex<State>>) -> impl Responder {
    if identity.is_none() {
        return web::Json(Err(Error::UnauthorizedUser));
    }

    let session_id = identity
        .unwrap()
        .id()
        .unwrap();

    let id = *state
        .lock()
        .await
        .users.get(&session_id)
        .unwrap();

    web::Json(
        routes::groups::Response::from(
            state.clone(),
            id
        ).await.map_err(
            |reason| Error::InternalServerError
        )
    )
}

#[get("/logout")]
async fn logout(id: Identity, state: web::Data<Mutex<State>>) -> impl Responder {
    let session_id = id
        .id()
        .unwrap();

    id.logout();

    let users = &mut state.lock().await.users;
    users.remove(&session_id);

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let repo = Repository::init()
        .await
        .expect("Database connection failed");

    let state = web::Data::new(Mutex::new(State::new(repo)));

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
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .supports_credentials()
                )
                .wrap(middleware::NormalizePath::trim())
                .wrap(middleware::Logger::default()),
        )
            .app_data(state.clone());
    };

    Ok(config.into())
}
