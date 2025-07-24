use std::fs;
use std::io::{self, Write};
use chrono::{DateTime, Local};

struct FileManager {
    current_directory: String,
}

impl FileManager {
    fn new() -> Self {
        FileManager {
            current_directory: String::from("./"),
        }
    }

    fn read_file(&self, filename: &str) -> Result<String, io::Error> {
        let path = format!("{}{}", self.current_directory, filename);
        fs::read_to_string(path)
    }

    fn write_file(&self, filename: &str, content: &str) -> Result<(), io::Error> {
        let path = format!("{}{}", self.current_directory, filename);
        fs::write(path, content)
    }

    fn modify_file(&self, filename: &str, new_content: &str) -> Result<(), io::Error> {
        let path = format!("{}{}", self.current_directory, filename);
        let mut existing_content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => String::new(),
        };
        existing_content.push_str("\n");
        existing_content.push_str(new_content);
        fs::write(path, existing_content)
    }

    fn delete_file(&self, filename: &str) -> Result<(), io::Error> {
        let path = format!("{}{}", self.current_directory, filename);
        fs::remove_file(path)
    }

    fn list_files(&self) -> Result<Vec<String>, io::Error> {
        let mut files = Vec::new();
        for entry in fs::read_dir(&self.current_directory)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    files.push(filename.to_string());
                }
            }
        }
        Ok(files)
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    while input.trim().is_empty() {
        input.clear();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
    }
    input.trim().to_string()
}

fn main() {
    let file_manager = FileManager::new();
    let now: DateTime<Local> = Local::now();
    
    println!("Gestionnaire de Fichiers - Démarré le {}", now.format("%Y-%m-%d %H:%M:%S"));
    println!("Répertoire courant: {}", file_manager.current_directory);
    
    loop {
        println!("\n=== MENU PRINCIPAL ===");
        println!("1. Lire un fichier");
        println!("2. Écrire un fichier");
        println!("3. Modifier un fichier");
        println!("4. Supprimer un fichier");
        println!("5. Lister les fichiers");
        println!("6. Quitter");

        let choice = get_user_input("Votre choix (1-6): ");

        match choice.as_str() {
            "1" => {
                let filename = get_user_input("Nom du fichier à lire: ");
                match file_manager.read_file(&filename) {
                    Ok(content) => {
                        println!("\nContenu du fichier '{}':", filename);
                        println!("{}", content);
                    }
                    Err(e) => println!("Erreur lors de la lecture: {}", e),
                }
            }
            
            "2" => {
                let filename = get_user_input("Nom du fichier à créer: ");
                println!("Entrez le contenu (tapez 'FIN' sur une ligne pour terminer):");

                let mut content = String::new();
                loop {
                    let line = get_user_input("");
                    if line == "FIN" {
                        break;
                    }
                    content.push_str(&line);
                    content.push('\n');
                }
                
                match file_manager.write_file(&filename, &content) {
                    Ok(()) => println!("Fichier '{}' créé avec succès!", filename),
                    Err(e) => println!("Erreur lors de l'écriture: {}", e),
                }
            }
            
            "3" => {
                let filename = get_user_input("Nom du fichier à modifier: ");
                let new_content = get_user_input("Contenu à ajouter: ");
                
                match file_manager.modify_file(&filename, &new_content) {
                    Ok(()) => println!("Fichier '{}' modifié avec succès!", filename),
                    Err(e) => println!("Erreur lors de la modification: {}", e),
                }
            }
            
            "4" => {
                let filename = get_user_input("Nom du fichier à supprimer: ");
                
                let mut confirmation = String::new();
                while confirmation != "oui" && confirmation != "non" {
                    confirmation = get_user_input(&format!("Êtes-vous sûr de vouloir supprimer '{}'? (oui/non): ", filename));
                }
                
                if confirmation == "oui" {
                    match file_manager.delete_file(&filename) {
                        Ok(()) => println!("Fichier '{}' supprimé avec succès!", filename),
                        Err(e) => println!("Erreur lors de la suppression: {}", e),
                    }
                } else {
                    println!("Suppression annulée");
                }
            }
            
            "5" => {
                match file_manager.list_files() {
                    Ok(files) => {
                        if files.is_empty() {
                            println!("Aucun fichier trouvé dans le répertoire");
                        } else {
                            println!("\nFichiers dans le répertoire:");
                            for (index, file) in files.iter().enumerate() {
                                println!("  {}. {}", index + 1, file);
                            }
                        }
                    }
                    Err(e) => println!("Erreur lors du listage: {}", e),
                }
            }

            "6" => {
                println!("Au revoir !");
                break;
            }
            
            _ => println!("Choix invalide, veuillez réessayer"),
        }
    }
}
