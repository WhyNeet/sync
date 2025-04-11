use std::{env, sync::Arc};

use auth::session::store::{
    SessionManager, impls::scylla::ScyllaSessionStore, integration::axum::session_middleware,
};
use axum::{Extension, Router, http::HeaderValue, middleware};
use identity::{handlers, state::AppState};
use reqwest::{
    Method,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, AUTHORIZATION, CONNECTION, CONTENT_LENGTH,
        CONTENT_TYPE, COOKIE, HOST, ORIGIN, REFERER, UPGRADE, USER_AGENT,
    },
};
use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    {
        unsafe {
            std::env::set_var("SCYLLA_URI", "127.0.0.1:9043");
            std::env::set_var(
                "SESSION_SIGNING_KEY",
                "OX0w0kHPRcxE3oD1Y2vw0Kfa8ZYLvgDt2oq/78yJFYJBev2uiuAKyKUrQgUP94UppV33bm+DKLYpDcFhwBE6UA==",
            );
            std::env::set_var("USERS_SERVICE_URI", "localhost:8080");
            std::env::set_var("KAFKA_BROKER_URI", "localhost:9092");
        };
    };

    #[cfg(not(debug_assertions))]
    {
        if !std::fs::exists(".log")? {
            std::fs::create_dir(".log")?;
        }
    };

    #[cfg(not(debug_assertions))]
    let (file_layer, _guard) = {
        let file_appender = tracing_appender::rolling::daily(".log", "identity.log");
        let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

        (
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking_appender)
                .with_ansi(false),
            guard,
        )
    };

    let stdout_layer = tracing_subscriber::fmt::layer().pretty();

    let subscriber = tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(stdout_layer);

    #[cfg(not(debug_assertions))]
    let subscriber = subscriber.with(file_layer);

    subscriber.init();

    let cors_layer = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://localhost.com".parse::<HeaderValue>().unwrap(),
        ])
        .allow_credentials(true)
        .allow_headers([
            AUTHORIZATION,
            COOKIE,
            ACCEPT,
            CONTENT_TYPE,
            HOST,
            USER_AGENT,
            ACCEPT_ENCODING,
            CONNECTION,
            UPGRADE,
            CONTENT_LENGTH,
            CONNECTION,
            ACCEPT_LANGUAGE,
            REFERER,
            ORIGIN,
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::DELETE]);

    let app_state = AppState::new().await.unwrap();
    let session_manager = Arc::new(SessionManager::new(
        ScyllaSessionStore::new(
            Arc::clone(&app_state.db),
            "ks".to_string(),
            "sessions".to_string(),
        )
        .await
        .unwrap(),
        env::var("SESSION_SIGNING_KEY").unwrap().as_bytes(),
    ));

    let app = Router::new()
        .merge(common::handlers::default_router())
        .merge(handlers::auth::router())
        .with_state(Arc::new(app_state))
        .layer(
            TraceLayer::new_for_http().make_span_with(
                DefaultMakeSpan::default()
                    .include_headers(true)
                    .level(Level::DEBUG),
            ),
        )
        .layer(middleware::from_fn(
            session_middleware::<ScyllaSessionStore>,
        ))
        .layer(Extension(session_manager))
        .layer(cors_layer);

    let listener = TcpListener::bind(
        #[cfg(not(debug_assertions))]
        "0.0.0.0:8080",
        #[cfg(debug_assertions)]
        "0.0.0.0:8081",
    )
    .await?;

    tracing::info!("starting server...");

    axum::serve(listener, app).await
}
