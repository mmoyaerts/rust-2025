# README
Gestionnaire de fichiers en Rust permettant les opérations CRUD complètes sur les fichiers avec horodatage.

## Fonctionnalités
* **Création de fichier** avec :
  * Nom personnalisé
  * Horodatage automatique de la création
  * Ajout à la liste des fichiers créés
* **Écriture** dans un fichier (écrase le contenu existant)
* **Lecture** complète du contenu d'un fichier
* **Modification** par ajout de contenu (append)
* **Suppression définitive** avec confirmation utilisateur
* **Informations détaillées** d'un fichier :
  * Nom, chemin, taille
  * Date de création et modification
* **Listage** de tous les fichiers du répertoire courant
* **Statut** du gestionnaire avec historique des opérations

## Organisation du code
* **Struct `FileManager`**
  * Champs : `current_directory: String`, `files_created: Vec<String>`, `last_operation: Option<String>`
  * Méthodes :
    * `new() -> Self`
    * `create_file(filename) -> io::Result<()>`
    * `write_file(filename, content) -> io::Result<()>`
    * `read_file(filename) -> io::Result<String>`
    * `append_to_file(filename, content) -> io::Result<()>`
    * `delete_file(filename) -> io::Result<()>`
    * `get_file_info(filename) -> io::Result<FileInfo>`
    * `list_files() -> Vec<String>`
    * `show_status()`

* **Struct `FileInfo`**
  * Champs : `name: String`, `path: String`, `size: u64`, `created_at: DateTime<Local>`, `modified_at: DateTime<Local>`
  * Méthodes :
    * `display()`

* **Boucle principale (`main`)**
  * Menu en boucle avec commandes textuelles
  * Gestion des cas avec `match` sur les commandes
  * Parcours des fichiers avec boucle `while` et compteur

## Concepts Rust exploités
* **Ownership & Borrowing**
  * Passage par référence (`&str`, `&self`)
  * Transfert de propriété avec `String::new()` et `to_string()`
  * Gestion mémoire automatique

* **Pattern Matching**
  * `match` pour les commandes utilisateur
  * `match` pour la gestion d'erreurs `Result<T, E>`
  * Patterns sur `Option<T>` pour `last_operation`

* **Structures et Implémentations**
  * `impl` blocks pour `FileManager` et `FileInfo`
  * Méthodes avec `&mut self` pour modification d'état
  * Méthodes avec `&self` pour lecture seule

* **Gestion d'erreurs**
  * Type `Result<T, E>` pour toutes les opérations fichiers
  * Propagation d'erreurs avec `?` operator
  * Gestion explicite avec `match` sur `Result`

* **Boucles et itération**
  * `loop` principal pour l'interface utilisateur
  * `while` avec compteur pour l'affichage de liste
  * `for` loop pour parcourir les entrées de répertoire

* **Collections**
  * `Vec<String>` pour stocker les fichiers créés
  * `Vec<&str>` pour parser les commandes utilisateur

* **Gestion du temps**
  * Utilisation de `chrono` pour les timestamps
  * `DateTime<Local>` pour les dates locales
  * Formatage personnalisé avec `format()`

* **I/O et système de fichiers**
  * Module `std::fs` pour les opérations fichiers
  * Module `std::io` pour les entrées/sorties
  * Gestion des permissions et métadonnées

## Dépendances
```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
```

## Utilisation
```bash
cargo add chrono --features serde
cargo run
```

### Commandes disponibles
- `create <nom>` - Créer un nouveau fichier
- `write <nom>` - Écrire du contenu dans un fichier
- `read <nom>` - Lire le contenu d'un fichier
- `append <nom>` - Ajouter du contenu à un fichier
- `delete <nom>` - Supprimer définitivement un fichier
- `info <nom>` - Afficher les informations d'un fichier
- `list` - Lister tous les fichiers du répertoire
- `status` - Afficher le statut du gestionnaire
- `quit` - Quitter l'application