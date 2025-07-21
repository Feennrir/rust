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

**NOTE :** Qu'est ce qu'un `mut` ?
Le mot-clé `mut` en Rust indique qu'une variable est mutable, c'est-à-dire qu'elle peut être modifiée après sa déclaration.


### Loops
Rust propose plusieurs types de boucles, notamment `loop`, `while` et `for`.
Voici un exemple de boucle `loop` :
```rust
fn main() {
    let mut compteur = 0;
    loop {
        compteur += 1;
        if compteur == 5 {
            break; // Sort de la boucle
        }
        println!("Compteur : {}", compteur);
    }
}
```

### Les structures
Les structures (`struct`) permettent de créer des types de données personnalisés. Voici un exemple :
```rust
struct Point {
    x: f64,
    y: f64,
}
impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
fn main() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("La distance entre p1 et p2 est : {}", p1.distance(&p2));
}
```

### Fonctions associées
Les fonctions associées sont des fonctions définies dans une structure qui peuvent être appelées sans instance de la structure. Elles sont souvent utilisées pour créer des constructeurs. Voici un exemple :
```rust
struct Circle {
    radius: f64,
}
impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    fn area(&self) -> f64 {     
        std::f64::consts::PI * self.radius.powi(2)
    }
}
```

```rust
struct Compteur {
    valeur: i32,
}
impl Compteur {
    fn new() -> Compteur {
        Compteur { valeur: 0 }
    }
    fn incrementer(&mut self) { // &mut self : prend une référence mutable à self, ce qui permet de modifier l'instance de la structure
        self.valeur += 1;
    }
    fn afficher(&self) {
        println!("Valeur actuelle : {}", self.valeur);
    }
}
```

- `&self` : représente une référence à l'instance actuelle de la structure, permettant d'accéder à ses données sans la posséder. Lecture uniquement.
- `&mut self` : représente une référence mutable à l'instance actuelle, permettant de modifier ses données. Écriture autorisée.
- `self` : prend la possession de l'instance, ce qui signifie que l'instance n'est plus accessible après l'appel de la méthode. Utilisé pour consommer l'instance.