use std::sync::Arc;
use std::sync::Mutex;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use chrono::Utc;
use std::time::Instant;

#[tokio::main]
async fn main() {
    // Chemin du fichier de log
    let log_file_path = "logs/server.log";

    // Créer le dossier logs/ si nécessaire
    std::fs::create_dir_all("logs").expect("Impossible de créer le dossier logs");

    // Créer ou ouvrir le fichier de log
    let log_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .expect("Impossible d'ouvrir ou de créer le fichier de log"),
    ));

    // Démarrer le serveur TCP
    let listener = TcpListener::bind("127.0.0.1:10000")
        .await
        .expect("Impossible de démarrer le serveur");

    println!("Serveur de journalisation démarré sur 127.0.0.1:10000");

    // Canal pour gérer les messages entrants
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Tâche pour écrire les logs dans le fichier
    let log_file_clone = Arc::clone(&log_file);
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let mut file = log_file_clone.lock().unwrap();
            writeln!(file, "{}", message).expect("Impossible d'écrire dans le fichier de log");
        }
    });

    // Mesurer le temps d'exécution des tâches
    let debut = Instant::now();

    // Accepter les connexions entrantes
    let mut tasks = vec![];
    loop {
        let (socket, addr) = listener.accept().await.expect("Erreur lors de l'acceptation d'une connexion");
        println!("Nouvelle connexion de {}", addr);

        let tx_clone = tx.clone();
        let task = tokio::spawn(async move {
            let mut reader = BufReader::new(socket);
            let mut line = String::new();

            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                let timestamp = Utc::now().to_rfc3339();
                let log_message = format!("[{}] {}", timestamp, line.trim());
                tx_clone.send(log_message).await.expect("Erreur lors de l'envoi du message");
                line.clear();
            }

            println!("Connexion fermée pour {}", addr);
        });

        tasks.push(task);

        // Si toutes les tâches doivent être attendues à un moment donné
        if tasks.len() >= 3 {
            let _ = tokio::join!(tasks.pop().unwrap(), tasks.pop().unwrap(), tasks.pop().unwrap());
            println!("Toutes les tâches terminées avec succès en {:?}", debut.elapsed());
        }
    }
}