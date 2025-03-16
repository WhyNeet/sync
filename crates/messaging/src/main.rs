use actix_web::{App, HttpServer, web};
use messaging::state::AppState;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

#[actix_web::main]
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

    CombinedLogger::init(vec![
        #[cfg(not(debug_assertions))]
        WriteLogger::new(LevelFilter::Debug, Config::default(), log_file.unwrap()),
        WriteLogger::new(LevelFilter::Debug, Config::default(), std::io::stdout()),
    ])
    .unwrap();

    let app_state = AppState::new().await.unwrap();
    let app_state = web::Data::new(app_state);

    HttpServer::new(move || App::new().app_data(app_state.clone()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
