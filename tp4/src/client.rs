use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Lancer plusieurs clients en parallèle
    let t1 = tokio::spawn(send_logs("Client A", vec!["log de A1", "log de A2", "log de A3"]));
    let t2 = tokio::spawn(send_logs("Client B", vec!["log de B1", "log de B2"]));
    let t3 = tokio::spawn(send_logs("Client C", vec!["log de C1", "log de C2", "log de C3", "log de C4"]));

    // Attendre que tous les clients terminent
    let _ = tokio::join!(t1, t2, t3);

    println!("Tous les clients ont terminé d'envoyer leurs logs.");
}

async fn send_logs(client_name: &str, messages: Vec<&str>) {
    // Connexion au serveur
    let mut stream = TcpStream::connect("127.0.0.1:10000")
        .await
        .expect(&format!("{}: Impossible de se connecter au serveur", client_name));

    println!("{}: Connecté au serveur", client_name);

    for message in messages {
        let log_message = format!("{}: {}", client_name, message);
        stream
            .write_all(log_message.as_bytes())
            .await
            .expect(&format!("{}: Impossible d'envoyer le message", client_name));
        stream.write_all(b"\n").await.expect("Erreur lors de l'envoi du retour à la ligne");

        println!("{}: Message envoyé -> {}", client_name, log_message);

        // Pause entre les messages pour simuler un délai
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    println!("{}: Déconnexion", client_name);
}