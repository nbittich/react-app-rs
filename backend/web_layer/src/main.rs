use std::net::SocketAddr;

use axum::Router;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod router;
#[tokio::main]
async fn main() {
    setup_tracing();

    let pool_db = db_layer::db::get_pool();

    let addr = std::env::var("ADDR").unwrap_or_else(|_| String::from("0.0.0.0:8080"));

    let socket_addr: SocketAddr = addr.parse().expect("unable to parse socket address");

    let app = Router::new()
        .merge(router::todos_list(pool_db.clone()))
        .merge(router::delete_todo(pool_db.clone()))
        .merge(router::new_todo(pool_db.clone()));

    tracing::info!("listening on {:?}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
