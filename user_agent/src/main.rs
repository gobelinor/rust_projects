use reqwest::Client;
use reqwest::header::USER_AGENT;
use std::env;


async fn get_response(user_agent: &String, url: &String, client: reqwest::Client) -> Result<u16, reqwest::Error> {
    // Faire une requête HTTP avec un User-Agent spécifié
    let response = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;
    Ok(response.status().as_u16())
}

#[tokio::main]
async fn main() -> () {
    // Récupérer les arguments User-Agent et URL
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <user_agent> <url>", args[0]);
        std::process::exit(1);
    }
    let user_agent = &args[1];
    let url = &args[2];
    // Créer un client HTTP
    let client = Client::new();
    
    match get_response(user_agent, url, client).await {
        Ok(status) => println!("Response status: {} ! You successfully sent a request to {}, with {} as User-Agent !", status, url, user_agent),
        Err(msg) => eprintln!("Error : {}", msg)
    }
}

