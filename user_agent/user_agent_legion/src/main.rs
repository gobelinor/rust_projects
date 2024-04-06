// DISCLAIMER : DON'T USE THIS ON SOMETHING YOU DON'T HAVE THE RIGHT TO USE IT ON  
// This is a simple program that sends multiple requests to a website with a specified User-Agent
// It uses the reqwest library to send HTTP requests and tokio to handle async tasks
// It takes two arguments : the User-Agent and the URL to send the requests to

// Usage : cargo run <user_agent> <url>

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
    // Créer un client HTTP[S]
    let client = Client::new();
   
    //les requêtes sont des tasks (handle) que l'on fait spawn avec tokio::spawn
    //vecteur handles pour stocker les tasks "handle"
    let mut handles = vec![];
    for i in 0.. { //wtf cest beaucoup de requêtes 
        //clonage des variables (est ce necessaire ? peut etre trouver une autre façon plus opti) 
        let user_agent = user_agent.clone();
        let url = url.clone();
        let client = client.clone();
        //creation de la task 
        let handle = tokio::spawn(async move {
            match get_response(&user_agent, &url, client).await {
                Ok(status) => println!("Request [{}] response status: {} ! You successfully sent a request to {}, with {} as User-Agent !", i, status, url, user_agent),
                Err(msg) => eprintln!("Request [{}] Error : {}", i, msg)
            }
        });
        //ajout de la task au vecteur
        handles.push(handle);
    }
}
