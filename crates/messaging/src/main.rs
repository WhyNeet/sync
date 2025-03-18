use std::sync::Arc;

use axum::{Router, routing::any};
use messaging::{handlers, state::AppState};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    {
        unsafe { std::env::set_var("SCYLLA_URI", "127.0.0.1:9042") };
    };

    #[cfg(not(debug_assertions))]
    let log_file = {
        if !fs::exists(".log")? {
            fs::create_dir(".log")?;
        }
        File::options()
            .append(true)
            .create(true)
            .open(".log/messaging.log")?
    };

    #[cfg(not(debug_assertions))]
    let file_layer = {
        let file_appender = tracing_appender::rolling::daily(".log", "messaging.log");
        let (non_blocking_appender, _guard) = non_blocking(file_appender);

        let file_layer: Layer<
            Layered<
                Layer<
                    Layered<EnvFilter, tracing_subscriber::Registry>,
                    tracing_subscriber::fmt::format::Pretty,
                    tracing_subscriber::fmt::format::Format<
                        tracing_subscriber::fmt::format::Pretty,
                    >,
                >,
                Layered<EnvFilter, tracing_subscriber::Registry>,
            >,
            tracing_subscriber::fmt::format::DefaultFields,
            tracing_subscriber::fmt::format::Format,
            non_blocking::NonBlocking,
        > = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking_appender)
            .with_ansi(false);
        file_layer
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

    let app_state = AppState::new().await.unwrap();

    let app = Router::new()
        .route("/chat", any(handlers::chat))
        .with_state(Arc::new(app_state))
        .layer(
            TraceLayer::new_for_http().make_span_with(
                DefaultMakeSpan::default()
                    .include_headers(true)
                    .level(Level::DEBUG),
            ),
        );

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    tracing::info!("starting server...");

    axum::serve(listener, app).await
}
