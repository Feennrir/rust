use std::io;

fn main() {
    println!("=== Système de Gestion de Comptes Bancaires ===\n");

    let mut noms = [
        "Alice Dupont".to_string(),
        "Bob Martin".to_string(),
        "Claire Durand".to_string(),
    ];
    
    let mut soldes = vec![1000.0, 500.0, 750.0];

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
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        
        let choix: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide!");
                continue;
            }
        };

        // Traiter le choix
        match choix {
            1 => afficher_solde(&noms, &soldes),
            2 => effectuer_retrait(&noms, &mut soldes),
            3 => lister_comptes(&noms, &soldes),
            4 => {
                println!("Au revoir!");
                break;
            }
            _ => println!("Option invalide, veuillez choisir entre 1 et 4."),
        }
    }
}

fn afficher_solde(noms: &[String], soldes: &[f64]) {
    println!("\n--- Afficher Solde ---");
    
    if noms.is_empty() {
        println!("Aucun compte disponible.");
        return;
    }

    println!("Choisissez un compte:");
    for (index, nom) in noms.iter().enumerate() {
        println!("{}. {}", index + 1, nom);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    
    let choix: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= noms.len() => num - 1,
        _ => {
            println!("Choix invalide!");
            return;
        }
    };

    println!("Solde de {}: {:.2}€", noms[choix], soldes[choix]);
}

fn effectuer_retrait(noms: &[String], soldes: &mut [f64]) {
    println!("\n--- Retrait ---");
    
    if noms.is_empty() {
        println!("Aucun compte disponible.");
        return;
    }

    println!("Choisissez un compte:");
    for (index, nom) in noms.iter().enumerate() {
        println!("{}. {} (Solde: {:.2}€)", index + 1, nom, soldes[index]);
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    
    let choix: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= noms.len() => num - 1,
        _ => {
            println!("Choix invalide!");
            return;
        }
    };

    println!("Montant à retirer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    
    let montant: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Montant invalide!");
            return;
        }
    };

    if montant <= soldes[choix] && montant > 0.0 {
        soldes[choix] -= montant;
        println!("Retrait de {:.2}€ effectué avec succès!", montant);
        println!("Nouveau solde: {:.2}€", soldes[choix]);
    } else {
        println!("Retrait impossible (solde insuffisant ou montant invalide).");
    }
}

fn lister_comptes(noms: &[String], soldes: &[f64]) {
    println!("\n--- Liste des Comptes ---");
    
    if noms.is_empty() {
        println!("Aucun compte enregistré.");
        return;
    }

    for (index, nom) in noms.iter().enumerate() {
        println!("{}. {} - Solde: {:.2}€", index + 1, nom, soldes[index]);
    }
}