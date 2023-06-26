mod client;
mod server;
mod state;

use actix::{Actor, Addr};
use actix_web_actors::ws;
use server::GameServer;

use actix_web::*;

use crate::client::GameClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = GameServer::default().start();
    HttpServer::new(move || {
        App::new()
            .service(ws_route)
            .app_data(web::Data::new(server.clone()))
    })
    .bind(("0.0.0.0", 3100))?
    .run()
    .await
}

#[get("/")]
async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<GameServer>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(GameClient::new(server.get_ref().clone()), &req, stream);
    println!("{:?}", resp);
    resp
}
