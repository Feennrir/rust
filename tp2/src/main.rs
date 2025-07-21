use std::io;

// Structure pour représenter un compte bancaire
struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    // Méthode pour créer un nouveau compte
    fn nouveau(nom: String, solde: f64) -> CompteBancaire {
        CompteBancaire { nom, solde }
    }

    // Méthode pour effectuer un retrait
    fn retrait(&mut self, montant: f64) -> bool {
        if montant <= self.solde && montant > 0.0 {
            self.solde -= montant;
            true
        } else {
            false
        }
    }

    // Méthode pour obtenir le solde
    fn obtenir_solde(&self) -> f64 {
        self.solde
    }

    // Méthode pour obtenir le nom
    fn obtenir_nom(&self) -> &str {
        &self.nom
    }

    // Méthode pour afficher les informations du compte
    fn afficher_infos(&self) {
        println!("{} - Solde: {:.2}€", self.nom, self.solde);
    }
}

fn main() {
    // TP2 - Système de compte bancaire avec struct et méthodes
    println!("=== Système de Gestion de Comptes Bancaires ===\n");

    // Créer des comptes avec la struct
    let mut comptes = vec![
        CompteBancaire::nouveau("Alice Dupont".to_string(), 1000.0),
        CompteBancaire::nouveau("Bob Martin".to_string(), 500.0),
        CompteBancaire::nouveau("Claire Durand".to_string(), 750.0),
    ];

    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    loop {
        // Afficher le menu
        println!("\n--- MENU ---");
        for (index, option) in options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        print!("\nChoisissez une option (1-4): ");

        // Lire l'entrée utilisateur
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur de lecture");

        let choix: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide!");
                continue;
            }
        };

        // Traiter le choix
        match choix {
            1 => afficher_solde(&comptes),
            2 => effectuer_retrait(&mut comptes),
            3 => lister_comptes(&comptes),
            4 => {
                println!("Au revoir!");
                break;
            }
            _ => println!("Option invalide, veuillez choisir entre 1 et 4."),
        }
    }
}

// Fonction pour afficher le solde d'un compte
fn afficher_solde(comptes: &[CompteBancaire]) {
    println!("\n--- Afficher Solde ---");

    if comptes.is_empty() {
        println!("Aucun compte disponible.");
        return;
    }

    println!("Choisissez un compte:");
    for (index, compte) in comptes.iter().enumerate() {
        println!("{}. {}", index + 1, compte.obtenir_nom());
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    let choix: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= comptes.len() => num - 1,
        _ => {
            println!("Choix invalide!");
            return;
        }
    };

    let compte = &comptes[choix];
    println!(
        "Solde de {}: {:.2}€",
        compte.obtenir_nom(),
        compte.obtenir_solde()
    );
}

// Fonction pour effectuer un retrait
fn effectuer_retrait(comptes: &mut [CompteBancaire]) {
    println!("\n--- Retrait ---");

    if comptes.is_empty() {
        println!("Aucun compte disponible.");
        return;
    }

    println!("Choisissez un compte:");
    for (index, compte) in comptes.iter().enumerate() {
        println!(
            "{}. {} (Solde: {:.2}€)",
            index + 1,
            compte.obtenir_nom(),
            compte.obtenir_solde()
        );
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    let choix: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= comptes.len() => num - 1,
        _ => {
            println!("Choix invalide!");
            return;
        }
    };

    println!("Montant à retirer:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    let montant: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Montant invalide!");
            return;
        }
    };

    // Utiliser la méthode retrait de la struct
    if comptes[choix].retrait(montant) {
        println!("Retrait de {:.2}€ effectué avec succès!", montant);
        println!("Nouveau solde: {:.2}€", comptes[choix].obtenir_solde());
    } else {
        println!("Retrait impossible (solde insuffisant ou montant invalide).");
    }
}

// Fonction pour lister tous les comptes
fn lister_comptes(comptes: &[CompteBancaire]) {
    println!("\n--- Liste des Comptes ---");

    if comptes.is_empty() {
        println!("Aucun compte enregistré.");
        return;
    }

    for (index, compte) in comptes.iter().enumerate() {
        print!("{}. ", index + 1);
        compte.afficher_infos();
    }
}
