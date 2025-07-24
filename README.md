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

### Projet : Gestionnaire de Fichiers


#### Structure du projet
Le gestionnaire de fichiers utilise une structure `FileManager` qui encapsule les opérations sur les fichiers :

```rust
struct FileManager {
    current_directory: String,
}

impl FileManager {
    fn new() -> Self {
        FileManager {
            current_directory: String::from("./"),
        }
    }
}
```

#### Fonctionnalités implémentées

**1. Lecture de fichiers**
```rust
fn read_file(&self, filename: &str) -> Result<String, io::Error> {
    let path = format!("{}{}", self.current_directory, filename);
    fs::read_to_string(path)
}
```
- Utilise `fs::read_to_string()` pour lire le contenu complet d'un fichier
- Retourne un `Result<String, io::Error>` pour gérer les erreurs

**2. Écriture de fichiers**
```rust
fn write_file(&self, filename: &str, content: &str) -> Result<(), io::Error> {
    let path = format!("{}{}", self.current_directory, filename);
    fs::write(path, content)
}
```
- Utilise `fs::write()` pour créer ou écraser un fichier
- Retourne `Result<(), io::Error>` pour indiquer le succès ou l'échec

**3. Modification de fichiers**
```rust
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
```
- Lit d'abord le contenu existant avec gestion d'erreur via `match`
- Ajoute le nouveau contenu à la fin du fichier existant

**4. Suppression de fichiers**
```rust
fn delete_file(&self, filename: &str) -> Result<(), io::Error> {
    let path = format!("{}{}", self.current_directory, filename);
    fs::remove_file(path)
}
```

**5. Listage des fichiers**
```rust
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
```
- Utilise `fs::read_dir()` pour parcourir le répertoire
- Filtre uniquement les fichiers (pas les dossiers)
- Utilise l'opérateur `?` pour la propagation d'erreurs

#### Gestion des entrées utilisateur
```rust
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Force l'affichage immédiat
    
    let mut input = String::new();
    while input.trim().is_empty() {
        input.clear();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
    }
    input.trim().to_string()
}
```
- Fonction utilitaire pour lire l'entrée utilisateur
- Utilise `io::stdout().flush()` pour forcer l'affichage du prompt
- Boucle jusqu'à obtenir une entrée non vide

#### Concepts Rust illustrés
- **Structures et implémentations** : Organisation du code avec `struct` et `impl`
- **Gestion d'erreurs** : Utilisation de `Result<T, E>` et de l'opérateur `?`
- **Pattern matching** : Utilisation de `match` pour gérer les cas
- **Ownership et borrowing** : Utilisation de références (`&self`, `&str`)
- **Collections** : Manipulation de `Vec<String>` et `String`
- **Modules externes** : Utilisation de `chrono` pour les dates
- **I/O et gestion de fichiers** : Opérations sur le système de fichiers

## Projet : Serveur de journalisation asynchrone

### Objectif
L'objectif de ce projet est d'implémenter un serveur de journalisation asynchrone capable de :
- Écouter sur un port TCP.
- Recevoir des messages texte de plusieurs clients simultanément.
- Enregistrer ces messages dans un fichier de log avec un horodatage.

### Structure du projet
Le projet est organisé comme suit :
```
journalisation_server/
|---src/
   |__main.rs       # Serveur de journalisation
   |__client.rs     # Clients pour tester le serveur
|--logs/
   |__server.log    # Fichier généré automatiquement pour stocker les logs
|--Cargo.toml       # Dépendances et configuration du projet
```

### Fonctionnement du serveur (`main.rs`)
1. **Démarrage du serveur** :
   - Le serveur écoute sur `127.0.0.1:10000` grâce à `TcpListener`.
   - Un fichier de log (`logs/server.log`) est créé automatiquement si nécessaire.

2. **Gestion des connexions** :
   - Chaque connexion client est gérée dans une tâche asynchrone indépendante grâce à `tokio::spawn`.
   - Les messages reçus des clients sont envoyés via un canal (`mpsc::channel`) pour être écrits dans le fichier de log.

3. **Écriture des logs** :
   - Les messages sont horodatés avec `chrono::Utc` et enregistrés dans le fichier `server.log` au format suivant :
     ```
     [2025-07-24T14:20:00Z] Client A: log de A1
     ```

4. **Gestion des tâches** :
   - Les tâches asynchrones sont stockées dans un vecteur et peuvent être attendues simultanément avec `tokio::join!`.

### Fonctionnement des clients (`client.rs`)
1. **Connexion au serveur** :
   - Les clients se connectent au serveur via `TcpStream`.

2. **Envoi des messages** :
   - Chaque client envoie une série de messages texte au serveur avec un délai simulé entre chaque message (`tokio::time::sleep`).

3. **Exemple de sortie dans le fichier `server.log`** :
   ```
   [2025-07-24T14:20:00Z] Client A: log de A1
   [2025-07-24T14:20:01Z] Client B: log de B1
   [2025-07-24T14:20:01Z] Client A: log de A2
   [2025-07-24T14:20:02Z] Client B: log de B2
   [2025-07-24T14:20:02Z] Client A: log de A3
   ```

### Concepts Rust illustrés
- **Asynchronisme avec Tokio** :
  - Utilisation de `tokio::spawn` pour lancer des tâches asynchrones.
  - Gestion des connexions TCP avec `TcpListener` et `TcpStream`.

- **Mutex et gestion des ressources partagées** :
  - Utilisation de `std::sync::Mutex` pour protéger l'accès concurrent au fichier de log.

- **Canaux pour la communication** :
  - Utilisation de `tokio::sync::mpsc` pour transmettre les messages des tâches au gestionnaire de logs.

- **Horodatage** :
  - Utilisation de `chrono::Utc` pour générer des timestamps au format ISO 8601.

### Instructions pour tester
1. **Lancer le serveur** :
   - Exécutez le serveur avec la commande :
     ```bash
     cargo run --bin tp4
     ```

2. **Lancer les clients** :
   - Dans un autre terminal, exécutez les clients avec la commande :
     ```bash
     cargo run --bin client
     ```

3. **Vérifier le fichier de log** :
   - Les messages envoyés par les clients seront enregistrés dans `logs/server.log`.

### Notes importantes
- **Gestion des erreurs** :
  - Le serveur et les clients gèrent les erreurs de connexion et d'écriture avec des messages explicites.
- **Simultanéité** :
  - Plusieurs clients peuvent se connecter et envoyer des messages en même temps sans bloquer le serveur.
- **Structure modulaire** :
  - Le code est organisé en deux fichiers (`main.rs` pour le serveur et `client.rs` pour les clients) pour une meilleure lisibilité et testabilité.