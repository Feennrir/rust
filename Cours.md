# Introduction à Rust

## Liens utiles
- [Documentation officielle de Rust](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Playground](https://play.rust-lang.org/)

## Installation de Rust
Pour installer Rust :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Cargo
Cargo est le gestionnaire de paquets et l'outil de compilation pour Rust. Il permet de gérer les dépendances, de compiler le code et de créer des paquets.

### Initialisation d'un projet
Pour créer un nouveau projet Rust, utilisez la commande suivante :
```bash
cargo new nom_du_projet
cd nom_du_projet
```

### Compilation et exécution
Pour compiler et exécuter le projet, utilisez :
```bash
cargo run
```

## Les bases de Rust

### Variables et types
Rust est un langage statiquement typé. Voici comment déclarer des variables :
```rust
let x: i32 = 5; // variable immuable
let mut y: i32 = 10; // variable mutable
y += 5; // modification de la variable mutable
```

### Fonctions
Les fonctions sont définies avec le mot-clé `fn`. Voici un exemple :
```rust
fn ajouter(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let resultat = ajouter(5, 10);
    println!("Le résultat est : {}", resultat);
}
```

### IF et boucles
Rust utilise des expressions conditionnelles et des boucles pour le contrôle de flux. Voici un exemple de boucle `for` et d'instruction `if` :
```rust
fn main() {
    for i in 0..5 {
        if i % 2 == 0 {
            println!("{} est pair", i);
        } else {
            println!("{} est impair", i);
        }
    }
}
```

### Iterateurs
Rust propose des itérateurs puissants pour parcourir les collections. Voici un exemple d'utilisation d'un itérateur :
```rust
fn main() {
    let options = ["Option 1", "Option 2", "Option 3"];
    for (i, option) in options.iter().enumerate() {
        println!("{}: {}", i+1, option);
    }
}
```

### Input utilisateur
Pour lire l'entrée de l'utilisateur, on utilise la bibliothèque standard :
```rust
use std::io;

fn main() {
    let mut input = String::new(); // Mut : variable mutable 
    println!("Entrez un nombre :");
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let input:usize = input.trim().parse().expect("Veuillez entrer un nombre");
    println!("Vous avez entré : {}", input);
    if input % 2 == 0 {
        println!("{} est pair", input);
    } else {
        println!("{} est impair", input);
    }
    println!("Fin du programme");
}
```