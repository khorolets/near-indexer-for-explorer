use actix_web::{web, App, HttpServer};
use tracing::info;

pub(crate) async fn init_server() -> Result<(), std::io::Error> {
    let port: u16 = std::env::var("HTTP_PORT")
        .unwrap_or_else(|_| String::from("3030"))
        .parse()
        .expect("Unable to parse `HTTP_PORT`");

    info!(
        target: crate::INDEXER_FOR_EXPLORER,
        "Starting metrics server on http://0.0.0.0:{port}"
    );

    HttpServer::new(|| App::new().service(web::resource("/")))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
