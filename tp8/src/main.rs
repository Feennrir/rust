use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;

fn main() {
    let mode = std::env::args().nth(1).expect("Spécifiez 'serveur' ou 'client' comme argument.");
    if mode == "serveur" {
        serveur();
    } else if mode == "client" {
        client();
    } else {
        eprintln!("Argument invalide. Utilisez 'serveur' ou 'client'.");
    }
}

fn serveur() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Impossible de lier le port.");
    println!("Serveur en écoute sur 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Erreur de connexion : {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("Nouvelle connexion : {}", stream.peer_addr().unwrap());

    let mut buffer = [0; 4];
    while let Ok(_) = stream.read_exact(&mut buffer) {
        let message_length = u32::from_be_bytes(buffer);
        let mut message_buffer = vec![0; message_length as usize];
        stream.read_exact(&mut message_buffer).expect("Erreur de lecture du message.");

        let message = str::from_utf8(&message_buffer).expect("Message non valide.");
        println!("Message reçu : {}", message);

        // Envoyer une confirmation
        let response = "Message reçu".as_bytes();
        let response_length = (response.len() as u32).to_be_bytes();
        stream.write_all(&response_length).expect("Erreur d'envoi de la réponse.");
        stream.write_all(response).expect("Erreur d'envoi de la réponse.");
    }
}

fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Impossible de se connecter au serveur.");
    println!("Connecté au serveur.");

    let message = std::env::args().nth(2).expect("Spécifiez un message à envoyer.");
    let message_bytes = message.as_bytes();
    let message_length = (message_bytes.len() as u32).to_be_bytes();

    // Envoyer le message
    stream.write_all(&message_length).expect("Erreur d'envoi de la longueur du message.");
    stream.write_all(message_bytes).expect("Erreur d'envoi du message.");

    // Lire la réponse
    let mut buffer = [0; 4];
    stream.read_exact(&mut buffer).expect("Erreur de lecture de la longueur de la réponse.");
    let response_length = u32::from_be_bytes(buffer);
    let mut response_buffer = vec![0; response_length as usize];
    stream.read_exact(&mut response_buffer).expect("Erreur de lecture de la réponse.");

    let response = str::from_utf8(&response_buffer).expect("Réponse non valide.");
    println!("Réponse du serveur : {}", response);
}
