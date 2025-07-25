use std::net::UdpSocket;
use std::str;

fn main() {
    // Choisir entre le client et le serveur
    let mode = std::env::args().nth(1).expect("Spécifiez 'client' ou 'serveur' comme argument.");
    if mode == "serveur" {
        serveur_dns();
    } else if mode == "client" {
        client_dns();
    } else {
        eprintln!("Argument invalide. Utilisez 'client' ou 'serveur'.");
    }
}

fn serveur_dns() {
    let socket = UdpSocket::bind("127.0.0.1:10001").expect("Impossible de lier le socket.");
    println!("Serveur DNS en écoute sur 127.0.0.1:10001");

    let mut buffer = [0; 512];
    let domaines = vec![
        ("example.com", "93.184.216.34"),
        ("localhost", "127.0.0.1"),
    ];

    loop {
        let (size, src) = socket.recv_from(&mut buffer).expect("Erreur lors de la réception.");
        println!("Requête reçue de {}", src);

        // Extraire le domaine demandé (simplifié pour ce TP)
        let requete = str::from_utf8(&buffer[..size]).unwrap_or("");
        println!("Domaine demandé : {}", requete);

        // Chercher l'adresse IP correspondante
        let reponse = domaines
            .iter()
            .find(|(domaine, _)| *domaine == requete)
            .map(|(_, ip)| *ip)
            .unwrap_or("Domaine inconnu");

        // Envoyer la réponse
        socket
            .send_to(reponse.as_bytes(), src)
            .expect("Erreur lors de l'envoi de la réponse.");
    }
}

fn client_dns() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Impossible de lier le socket.");
    let serveur = "127.0.0.1:10001";

    let domaine = std::env::args().nth(2).expect("Spécifiez un domaine à résoudre.");
    socket
        .send_to(domaine.as_bytes(), serveur)
        .expect("Erreur lors de l'envoi de la requête.");

    let mut buffer = [0; 512];
    let (size, _) = socket.recv_from(&mut buffer).expect("Erreur lors de la réception.");
    let reponse = str::from_utf8(&buffer[..size]).unwrap_or("Réponse invalide");

    println!("Adresse IP pour {}: {}", domaine, reponse);
}
