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
    let log_file_path = "logs/server.log";

    // Créer le dossier logs/ si nécessaire
    std::fs::create_dir_all("logs").expect("Impossible de créer le dossier logs");

    /**
     * Arc : Partage une donnée entre plusieurs threads ou tâches
     * Mutex : Protège l'accès concurrent au fichier de log
     * OpenOptions : Permet de configurer l'ouverture du fichier de log
     * - create(true) : Crée le fichier s'il n'existe pas
     * - append(true) : Ajoute les nouvelles entrées à la fin du fichier
     */
    let log_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true) // Créer le fichier s'il n'existe pas
            .append(true) // Ajouter à la fin du fichier au lieu de l'écraser
            .open(log_file_path) 
            .expect("Impossible d'ouvrir ou de créer le fichier de log"),
    ));

    // On démarre le serveur TCP sur le port 10000
    let listener = TcpListener::bind("127.0.0.1:10000")
        .await
        .expect("Impossible de démarrer le serveur");

    println!("Serveur de journalisation démarré sur 127.0.0.1:10000");

    // Canal pour transmettre les messages reçus des clients vers l'écriture dans le fichier de log.
    // mpsc : Multi-producer, single-consumer channel
    // tx : Transmetteur pour envoyer des messages
    // rx : Récepteur pour recevoir des messages
    // Le canal a une capacité de 100 messages
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Tâche pour écrire les logs dans le fichier
    let log_file_clone = Arc::clone(&log_file); // Cloner l'Arc pour le partager avec la tâche d'écriture
    tokio::spawn(async move { // Tâche asynchrone pour écrire dans le fichier de log
        while let Some(message) = rx.recv().await { // Recevoir les messages du canal
            let mut file = log_file_clone.lock().unwrap(); // Verrouiller le Mutex pour accéder au fichier de log
            writeln!(file, "{}", message).expect("Impossible d'écrire dans le fichier de log"); // Écrire le message dans le fichier
        }
    });

    // Mesurer le temps d'exécution des tâches
    let debut = Instant::now();

    /**
     * Boucle infinie pour accepter les connexions entrantes et lancer des taches qui vont gérer les messages des clients.
     * Chaque tâche va lire les messages envoyés par un client, ajouter un timestamp et les envoyer au canal pour écriture dans le fichier de log.
     */
    let mut tasks = vec![];
    loop { // Boucle infinie pour accepter les connexions
        let (socket, addr) = listener.accept().await.expect("Erreur lors de l'acceptation d'une connexion"); // Accepter une connexion entrante
        println!("Nouvelle connexion de {}", addr);

        let tx_clone = tx.clone(); // Cloner le transmetteur pour l'utiliser dans la tâche
        let task = tokio::spawn(async move { // Tâche asynchrone pour gérer la connexion
            let mut reader = BufReader::new(socket); // Créer un lecteur pour lire les données du socket
            let mut line = String::new(); // Chaîne pour stocker les lignes lues

            while reader.read_line(&mut line).await.unwrap_or(0) > 0 { // Lire une ligne du socket
                let timestamp = Utc::now().to_rfc3339(); // Obtenir le timestamp actuel
                let log_message = format!("[{}] {}", timestamp, line.trim()); // Formater le message de log avec le timestamp
                tx_clone.send(log_message).await.expect("Erreur lors de l'envoi du message"); // Envoyer le message au canal
                line.clear();
            }

            println!("Connexion fermée pour {}", addr);
        });

        tasks.push(task); // Ajouter la tâche à la liste des tâches en cours

        // Si on a au moins 3 tâches en cours, on attend qu'elles se terminent
        if tasks.len() >= 3 {
            let _ = tokio::join!(tasks.pop().unwrap(), tasks.pop().unwrap(), tasks.pop().unwrap());
            println!("Toutes les tâches terminées avec succès en {:?}", debut.elapsed());
        }
    }
}