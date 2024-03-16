// This is a simple HTTP gateway that forwards requests to a server and modifies the User-Agent header
// Usage: cargo run <user_agent>

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
use std::env;

async fn handle_request(req: Request<Body>, user_agent: String) -> Result<Response<Body>, hyper::Error> {

    // Créer une nouvelle requête avec le header modifié
    let modified_req = Request::builder()
        .method(req.method())
        .uri(req.uri())
        .version(req.version())
        .header("User-Agent", user_agent)
        .body(req.into_body())
        .unwrap();

    // Faire la requête modifiée au serveur d'origine
    let client = hyper::Client::new();
    client.request(modified_req).await
}

#[tokio::main]
async fn main() {
    // Lire les arguments de la ligne de commande
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <user_agent>", args[0]);
        return;
    }

    let user_agent = &args[1];

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc = make_service_fn(move |_conn| {
        let user_agent = user_agent.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle_request(req, user_agent.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Gateway HTTP en cours d'écoute sur http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Erreur du serveur: {}", e);
    }
}

