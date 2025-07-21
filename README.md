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
Cargo est le gestionnaire de paquets et l'outil de compilation pour Rust. Il permet de gerer les dependances, de compiler le code et de creer des paquets.

### Initialisation d'un projet
Pour creer un nouveau projet Rust, utilisez la commande suivante :
```bash
cargo new nom_du_projet
cd nom_du_projet
```

### Compilation et execution
Pour compiler et executer le projet, utilisez :
```bash
cargo run
```

## Les bases de Rust

### Variables et types
Rust est un langage statiquement type. Voici comment declarer des variables :
```rust
let x: i32 = 5; // variable immuable
let mut y: i32 = 10; // variable mutable
y += 5; // modification de la variable mutable
```

### Fonctions
Les fonctions sont definies avec le mot-cle `fn`. Voici un exemple :
```rust
fn ajouter(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let resultat = ajouter(5, 10);
    println!("Le resultat est : {}", resultat);
}
```

### IF et boucles
Rust utilise des expressions conditionnelles et des boucles pour le controle de flux. Voici un exemple de boucle `for` et d'instruction `if` :
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
Rust propose des iterateurs puissants pour parcourir les collections. Voici un exemple d'utilisation d'un iterateur :
```rust
fn main() {
    let options = ["Option 1", "Option 2", "Option 3"];
    for (i, option) in options.iter().enumerate() {
        println!("{}: {}", i+1, option);
    }
}
```

### Input utilisateur
Pour lire l'entree de l'utilisateur, on utilise la bibliotheque standard :
```rust
use std::io;

fn main() {
    let mut input = String::new(); // Mut : variable mutable 
    println!("Entrez un nombre :");
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    let input:usize = input.trim().parse().expect("Veuillez entrer un nombre");
    println!("Vous avez entre : {}", input);
    if input % 2 == 0 {
        println!("{} est pair", input);
    } else {
        println!("{} est impair", input);
    }
    println!("Fin du programme");
}
```

**NOTE :** Qu'est ce qu'un `mut` ?
Le mot-cle `mut` en Rust indique qu'une variable est mutable, c'est-a-dire qu'elle peut etre modifiee apres sa declaration.

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
Les structures (`struct`) permettent de creer des types de donnees personnalises. Voici un exemple :
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

### Fonctions associees
Les fonctions associees sont des fonctions definies dans une structure qui peuvent etre appelees sans instance de la structure. Elles sont souvent utilisees pour creer des constructeurs. Voici un exemple :
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
    fn incrementer(&mut self) { // &mut self : prend une reference mutable a self, ce qui permet de modifier l'instance de la structure
        self.valeur += 1;
    }
    fn afficher(&self) {
        println!("Valeur actuelle : {}", self.valeur);
    }
}
```

- `&self` : represente une reference a l'instance actuelle de la structure, permettant d'acceder a ses donnees sans la posseder. Lecture uniquement.
- `&mut self` : represente une reference mutable a l'instance actuelle, permettant de modifier ses donnees. Ecriture autorisee.
- `self` : prend la possession de l'instance, ce qui signifie que l'instance n'est plus accessible apres l'appel de la methode. Utilise pour consommer l'instance.

### Gestion des fichiers
Rust permet de manipuler les fichiers de maniere securisee en utilisant le systeme de gestion d'erreurs integre.

#### Ecriture dans un fichier
Pour ecrire des donnees dans un fichier, on utilise les modules `std::fs` et `std::io` :
```rust
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("output.txt")?;

    file.write_all(b"Hello, world!")?;
    file.write_all(b"\nThis is a test file.")?;

    println!("File written successfully!");

    Ok(())
}
```

**Notes importantes sur la gestion des erreurs :**
- `io::Result<()>` est un type de resultat qui peut etre soit `Ok(())`, soit `Err(e)`. Il est utilise pour gerer les erreurs lors de l'ecriture dans un fichier.
- `Ok(())` signifie que la fonction main s'est executee avec succes et a retourne un resultat vide.
- `Err(e)` signifie que la fonction main s'est terminee avec une erreur

```rust
use std::fs::File;
use std::io::{self, BufReader, Read};
fn main() -> io::Result<()> {
    let file = File::open("output.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;

    println!("File contents:\n{}", contents);

    Ok(())
}
```