use futures_util::{StreamExt, SinkExt};
use std::env;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{accept_async, connect_async};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let mode = env::args().nth(1).expect("Spécifiez 'serveur' ou 'client' comme argument.");
    if mode == "serveur" {
        serveur().await;
    } else if mode == "client" {
        client().await;
    } else {
        eprintln!("Argument invalide. Utilisez 'serveur' ou 'client'.");
    }
}

async fn serveur() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Impossible de lier le port.");
    println!("Serveur WebSocket en écoute sur ws://127.0.0.1:8080");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Nouvelle connexion : {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Erreur avec la connexion {}: {}", addr, e);
            }
        });
    }
}

async fn handle_connection(stream: TcpStream) -> tokio_tungstenite::tungstenite::Result<()> {
    let ws_stream = accept_async(stream).await.expect("Erreur lors du handshake WebSocket.");
    println!("Connexion WebSocket établie.");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg?;
        println!("Message reçu : {}", msg);

        if msg.is_text() || msg.is_binary() {
            write.send(msg).await?;
        }
    }

    println!("Connexion fermée.");
    Ok(())
}

async fn client() {
    let url = "ws://127.0.0.1:8080";
    let (ws_stream, _) = connect_async(url).await.expect("Impossible de se connecter au serveur.");
    println!("Connecté au serveur WebSocket : {}", url);

    let (mut write, mut read) = ws_stream.split();

    // Envoyer un message au serveur
    let message = "Bonjour, serveur WebSocket !";
    write.send(Message::Text(message.to_string())).await.expect("Erreur lors de l'envoi du message.");
    println!("Message envoyé : {}", message);

    // Lire les messages du serveur
    while let Some(msg) = read.next().await {
        let msg = msg.expect("Erreur lors de la réception du message.");
        println!("Message reçu du serveur : {}", msg);
    }
}
