use actix_web::{App, HttpServer};
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    HttpServer::new(|| App::new())
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
