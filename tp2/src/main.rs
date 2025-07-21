use std::io;

// Structure pour représenter un compte bancaire
#[derive(Debug, Clone)]
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

    // Points bonus: Méthode pour effectuer un dépôt (empêche les montants négatifs)
    fn depot(&mut self, montant: f64) -> bool {
        if montant > 0.0 {
            self.solde += montant;
            true
        } else {
            false
        }
    }

    // Points bonus: Méthode renommer qui renvoie un nouveau compte avec le nom changé
    fn renommer(&self, nouveau_nom: String) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom,
            solde: self.solde,
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

    // Nouvelle méthode pour vérifier si le compte est en découvert
    fn est_en_decouvert(&self) -> bool {
        self.solde < 0.0
    }

}

fn main() {
    // TP2 - Système de compte bancaire avec struct et méthodes
    println!("=== Système de Gestion de Comptes Bancaires ===\n");

    // Points bonus: Créer un Vec<CompteBancaire> pour gérer plusieurs comptes
    let mut comptes = vec![
        CompteBancaire::nouveau("Alice Dupont".to_string(), 1000.0),
        CompteBancaire::nouveau("Bob Martin".to_string(), 500.0),
        CompteBancaire::nouveau("Claire Durand".to_string(), 750.0),
    ];

    let options = [
        "Afficher solde",
        "Retrait",
        "Dépôt",
        "Renommer compte",
        "Liste comptes",
        "Créer nouveau compte",
        "Supprimer compte",
        "Transfert entre comptes",
        "Rechercher compte",
        "Statistiques bancaires",
        "Quitter",
    ];

    loop {
        // Afficher le menu
        println!("\n--- MENU ---");
        for (index, option) in options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        print!("\nChoisissez une option (1-{}): ", options.len());

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
            3 => effectuer_depot(&mut comptes),
            4 => renommer_compte(&mut comptes),
            5 => lister_comptes(&comptes),
            6 => creer_nouveau_compte(&mut comptes),
            7 => supprimer_compte(&mut comptes),
            8 => transferer_fonds(&mut comptes),
            9 => rechercher_compte(&comptes),
            10 => afficher_statistiques(&comptes),
            11 => {
                println!("Au revoir!");
                break;
            }
            _ => println!("Option invalide, veuillez choisir entre 1 et {}.", options.len()),
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
    // Points bonus: utilisation de .iter() et .enumerate()
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

    // Afficher si le compte est en découvert
    if compte.est_en_decouvert() {
        println!("⚠️ Ce compte est en découvert!");
    }
}

// Fonction pour effectuer un retrait
fn effectuer_retrait(comptes: &mut [CompteBancaire]) {
    println!("\n--- Retrait ---");

    if comptes.is_empty() {
        println!("Aucun compte disponible.");
        return;
    }

    println!("Choisissez un compte:");
    // Points bonus: utilisation de .iter() et .enumerate()
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

// Points bonus: Fonction pour effectuer un dépôt (empêche les montants négatifs)
fn effectuer_depot(comptes: &mut [CompteBancaire]) {
    println!("\n--- Dépôt ---");

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

    println!("Montant à déposer:");
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

    // Utiliser la méthode depot qui empêche les montants négatifs
    if comptes[choix].depot(montant) {
        println!("Dépôt de {:.2}€ effectué avec succès!", montant);
        println!("Nouveau solde: {:.2}€", comptes[choix].obtenir_solde());
    } else {
        println!("Dépôt impossible (montant négatif ou nul).");
    }
}

// Points bonus: Fonction pour renommer un compte
fn renommer_compte(comptes: &mut Vec<CompteBancaire>) {
    println!("\n--- Renommer Compte ---");

    if comptes.is_empty() {
        println!("Aucun compte disponible.");
        return;
    }

    println!("Choisissez un compte à renommer:");
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

    println!("Nouveau nom:");
    let mut nouveau_nom = String::new();
    io::stdin()
        .read_line(&mut nouveau_nom)
        .expect("Erreur de lecture");

    let nouveau_nom = nouveau_nom.trim().to_string();

    if nouveau_nom.is_empty() {
        println!("Le nom ne peut pas être vide!");
        return;
    }

    // Utiliser la méthode renommer qui renvoie un nouveau compte
    let nouveau_compte = comptes[choix].renommer(nouveau_nom.clone());
    comptes[choix] = nouveau_compte;

    println!("Compte renommé avec succès en: {}", nouveau_nom);
}

// Nouvelle fonction pour créer un nouveau compte
fn creer_nouveau_compte(comptes: &mut Vec<CompteBancaire>) {
    println!("\n--- Créer Nouveau Compte ---");

    println!("Nom du nouveau compte:");
    let mut nom = String::new();
    io::stdin()
        .read_line(&mut nom)
        .expect("Erreur de lecture");
    
    let nom = nom.trim().to_string();
    
    if nom.is_empty() {
        println!("Le nom ne peut pas être vide!");
        return;
    }

    // Vérifier si le nom existe déjà
    if comptes.iter().any(|compte| compte.obtenir_nom() == nom) {
        println!("Un compte avec ce nom existe déjà!");
        return;
    }

    println!("Solde initial:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    let solde: f64 = match input.trim().parse() {
        Ok(num) if num >= 0.0 => num,
        _ => {
            println!("Solde invalide! Le solde doit être positif ou nul.");
            return;
        }
    };

    let nouveau_compte = CompteBancaire::nouveau(nom.clone(), solde);
    comptes.push(nouveau_compte);
    
    println!("Compte '{}' créé avec succès avec un solde de {:.2}€!", nom, solde);
}

// Nouvelle fonction pour supprimer un compte
fn supprimer_compte(comptes: &mut Vec<CompteBancaire>) {
    println!("\n--- Supprimer Compte ---");

    if comptes.is_empty() {
        println!("Aucun compte disponible.");
        return;
    }

    println!("Choisissez un compte à supprimer:");
    for (index, compte) in comptes.iter().enumerate() {
        println!("{}. {} (Solde: {:.2}€)", index + 1, compte.obtenir_nom(), compte.obtenir_solde());
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

    let compte_a_supprimer = &comptes[choix];
    let nom = compte_a_supprimer.obtenir_nom().to_string();
    let solde = compte_a_supprimer.obtenir_solde();

    // Confirmation avant suppression
    println!("Êtes-vous sûr de vouloir supprimer le compte '{}' avec un solde de {:.2}€? (oui/non)", nom, solde);
    let mut confirmation = String::new();
    io::stdin()
        .read_line(&mut confirmation)
        .expect("Erreur de lecture");

    if confirmation.trim().to_lowercase() == "oui" {
        comptes.remove(choix);
        println!("Compte '{}' supprimé avec succès!", nom);
    } else {
        println!("Suppression annulée.");
    }
}

// Nouvelle fonction pour effectuer un transfert entre comptes
fn transferer_fonds(comptes: &mut Vec<CompteBancaire>) {
    println!("\n--- Transfert entre Comptes ---");

    if comptes.len() < 2 {
        println!("Il faut au moins 2 comptes pour effectuer un transfert.");
        return;
    }

    println!("Compte source (débiter):");
    for (index, compte) in comptes.iter().enumerate() {
        println!("{}. {} (Solde: {:.2}€)", index + 1, compte.obtenir_nom(), compte.obtenir_solde());
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    let source: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= comptes.len() => num - 1,
        _ => {
            println!("Choix invalide!");
            return;
        }
    };

    println!("Compte destination (créditer):");
    for (index, compte) in comptes.iter().enumerate() {
        if index != source {
            println!("{}. {} (Solde: {:.2}€)", index + 1, compte.obtenir_nom(), compte.obtenir_solde());
        }
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    let destination: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= comptes.len() && num - 1 != source => num - 1,
        _ => {
            println!("Choix invalide ou même compte que la source!");
            return;
        }
    };

    println!("Montant à transférer:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    let montant: f64 = match input.trim().parse() {
        Ok(num) if num > 0.0 => num,
        _ => {
            println!("Montant invalide!");
            return;
        }
    };

    // Effectuer le transfert
    if comptes[source].retrait(montant) {
        comptes[destination].depot(montant);
        println!("Transfert de {:.2}€ effectué avec succès!", montant);
        println!("De: {} (nouveau solde: {:.2}€)", comptes[source].obtenir_nom(), comptes[source].obtenir_solde());
        println!("Vers: {} (nouveau solde: {:.2}€)", comptes[destination].obtenir_nom(), comptes[destination].obtenir_solde());
    } else {
        println!("Transfert impossible (solde insuffisant sur le compte source).");
    }
}

// Nouvelle fonction pour rechercher un compte par nom
fn rechercher_compte(comptes: &[CompteBancaire]) {
    println!("\n--- Rechercher Compte ---");

    if comptes.is_empty() {
        println!("Aucun compte disponible.");
        return;
    }

    println!("Nom à rechercher (partiel ou complet):");
    let mut recherche = String::new();
    io::stdin()
        .read_line(&mut recherche)
        .expect("Erreur de lecture");

    let recherche = recherche.trim().to_lowercase();

    if recherche.is_empty() {
        println!("Veuillez saisir un nom à rechercher.");
        return;
    }

    // Utiliser les itérateurs pour filtrer les comptes
    let comptes_trouves: Vec<&CompteBancaire> = comptes
        .iter()
        .filter(|compte| compte.obtenir_nom().to_lowercase().contains(&recherche))
        .collect();

    if comptes_trouves.is_empty() {
        println!("Aucun compte trouvé pour '{}'", recherche);
    } else {
        println!("Comptes trouvés:");
        for compte in comptes_trouves {
            print!("• ");
            compte.afficher_infos();
        }
    }
}

// Nouvelle fonction pour afficher des statistiques
fn afficher_statistiques(comptes: &[CompteBancaire]) {
    println!("\n--- Statistiques Bancaires ---");

    if comptes.is_empty() {
        println!("Aucun compte disponible.");
        return;
    }

    let total_comptes = comptes.len();
    
    // Utiliser les itérateurs pour calculer les statistiques
    let solde_total: f64 = comptes.iter().map(|compte| compte.obtenir_solde()).sum();
    let solde_moyen = solde_total / total_comptes as f64;
    
    let solde_max = comptes
        .iter()
        .map(|compte| compte.obtenir_solde())
        .fold(f64::NEG_INFINITY, f64::max);
    
    let solde_min = comptes
        .iter()
        .map(|compte| compte.obtenir_solde())
        .fold(f64::INFINITY, f64::min);

    let comptes_en_decouvert = comptes
        .iter()
        .filter(|compte| compte.est_en_decouvert())
        .count();

    let compte_plus_riche = comptes
        .iter()
        .max_by(|a, b| a.obtenir_solde().partial_cmp(&b.obtenir_solde()).unwrap());

    println!("Statistiques générales:");
    println!("  • Nombre total de comptes: {}", total_comptes);
    println!("  • Solde total de la banque: {:.2}€", solde_total);
    println!("  • Solde moyen: {:.2}€", solde_moyen);
    println!("  • Solde maximum: {:.2}€", solde_max);
    println!("  • Solde minimum: {:.2}€", solde_min);
    println!("  • Comptes en découvert: {}", comptes_en_decouvert);
    
    if let Some(compte) = compte_plus_riche {
        println!("  • Compte le plus riche: {} ({:.2}€)", compte.obtenir_nom(), compte.obtenir_solde());
    }
}


// Fonction pour lister tous les comptes
fn lister_comptes(comptes: &[CompteBancaire]) {
    println!("\n--- Liste des Comptes ---");

    if comptes.is_empty() {
        println!("Aucun compte enregistré.");
        return;
    }

    // Points bonus: utilisation de .iter() et .enumerate()
    for (index, compte) in comptes.iter().enumerate() {
        print!("{}. ", index + 1);
        compte.afficher_infos();
        
        // Ajouter des indicateurs visuels
        if compte.est_en_decouvert() {
            println!("   ⚠️ En découvert");
        } else if compte.obtenir_solde() > 1000.0 {
            println!("   💰 Compte VIP");
        }
    }
}
