use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, Read};
use chrono::{DateTime, Local};

// Structure principale du gestionnaire de fichiers
#[derive(Debug, Clone)]
pub struct FileManager {
    current_directory: String,
    files_created: Vec<String>,
    last_operation: Option<String>,
}

// Structure pour représenter un fichier avec ses métadonnées
#[derive(Debug)]
pub struct FileInfo {
    name: String,
    path: String,
    size: u64,
    created_at: DateTime<Local>,
    modified_at: DateTime<Local>,
}

impl FileManager {
    // Constructeur
    pub fn new() -> Self {
        FileManager {
            current_directory: ".".to_string(),
            files_created: Vec::new(),
            last_operation: None,
        }
    }

    // Créer un nouveau fichier
    pub fn create_file(&mut self, filename: &str) -> io::Result<()> {
        let file_path = format!("{}/{}", self.current_directory, filename);
        
        match File::create(&file_path) {
            Ok(_) => {
                self.files_created.push(filename.to_string());
                self.last_operation = Some(format!("Cree: {} a {}", filename, Local::now().format("%Y-%m-%d %H:%M:%S")));
                println!("Fichier '{}' cree avec succes!", filename);
                Ok(())
            }
            Err(e) => {
                println!("Erreur lors de la creation du fichier: {}", e);
                Err(e)
            }
        }
    }

    // Écrire dans un fichier
    pub fn write_file(&mut self, filename: &str, content: &str) -> io::Result<()> {
        let file_path = format!("{}/{}", self.current_directory, filename);
        
        match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&file_path) {
            Ok(mut file) => {
                match file.write_all(content.as_bytes()) {
                    Ok(_) => {
                        self.last_operation = Some(format!("Ecrit dans: {} a {}", filename, Local::now().format("%Y-%m-%d %H:%M:%S")));
                        println!("Contenu ecrit dans '{}' avec succes!", filename);
                        Ok(())
                    }
                    Err(e) => {
                        println!("Erreur lors de l'ecriture: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                println!("Impossible d'ouvrir le fichier: {}", e);
                Err(e)
            }
        }
    }

    // Lire un fichier
    pub fn read_file(&mut self, filename: &str) -> io::Result<String> {
        let file_path = format!("{}/{}", self.current_directory, filename);
        
        match File::open(&file_path) {
            Ok(mut file) => {
                let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(_) => {
                        self.last_operation = Some(format!("Lu: {} a {}", filename, Local::now().format("%Y-%m-%d %H:%M:%S")));
                        println!("Contenu de '{}':\n{}", filename, content);
                        Ok(content)
                    }
                    Err(e) => {
                        println!("Erreur lors de la lecture: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                println!("Fichier '{}' introuvable: {}", filename, e);
                Err(e)
            }
        }
    }

    // Modifier un fichier (ajouter du contenu)
    pub fn append_to_file(&mut self, filename: &str, content: &str) -> io::Result<()> {
        let file_path = format!("{}/{}", self.current_directory, filename);
        
        match OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path) {
            Ok(mut file) => {
                match writeln!(file, "{}", content) {
                    Ok(_) => {
                        self.last_operation = Some(format!("Modifie: {} a {}", filename, Local::now().format("%Y-%m-%d %H:%M:%S")));
                        println!("Contenu ajoute a '{}' avec succes!", filename);
                        Ok(())
                    }
                    Err(e) => {
                        println!("Erreur lors de la modification: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                println!("Impossible d'ouvrir le fichier pour modification: {}", e);
                Err(e)
            }
        }
    }

    // Supprimer définitivement un fichier
    pub fn delete_file(&mut self, filename: &str) -> io::Result<()> {
        let file_path = format!("{}/{}", self.current_directory, filename);
        
        match fs::remove_file(&file_path) {
            Ok(_) => {
                self.files_created.retain(|f| f != filename);
                self.last_operation = Some(format!("Supprime: {} a {}", filename, Local::now().format("%Y-%m-%d %H:%M:%S")));
                println!("Fichier '{}' supprime definitivement!", filename);
                Ok(())
            }
            Err(e) => {
                println!("Erreur lors de la suppression: {}", e);
                Err(e)
            }
        }
    }

    // Obtenir les informations d'un fichier
    pub fn get_file_info(&self, filename: &str) -> io::Result<FileInfo> {
        let file_path = format!("{}/{}", self.current_directory, filename);
        
        match fs::metadata(&file_path) {
            Ok(metadata) => {
                let created: DateTime<Local> = metadata.created()?.into();
                let modified: DateTime<Local> = metadata.modified()?.into();
                
                Ok(FileInfo {
                    name: filename.to_string(),
                    path: file_path,
                    size: metadata.len(),
                    created_at: created,
                    modified_at: modified,
                })
            }
            Err(e) => Err(e)
        }
    }

    // Lister tous les fichiers du répertoire courant
    pub fn list_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.current_directory) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(filename) = entry.file_name().to_str() {
                                files.push(filename.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        files
    }

    // Afficher le statut du gestionnaire
    pub fn show_status(&self) {
        println!("\n=== STATUT DU GESTIONNAIRE DE FICHIERS ===");
        println!("Repertoire courant: {}", self.current_directory);
        println!("Fichiers crees: {:?}", self.files_created);
        
        match &self.last_operation {
            Some(op) => println!("Derniere operation: {}", op),
            None => println!("Aucune operation effectuee"),
        }
        
        let files = self.list_files();
        println!("Fichiers disponibles: {:?}", files);
        println!("=========================================\n");
    }
}

impl FileInfo {
    // Afficher les informations du fichier
    pub fn display(&self) {
        println!("\n=== INFORMATIONS DU FICHIER ===");
        println!("Nom: {}", self.name);
        println!("Chemin: {}", self.path);
        println!("Taille: {} octets", self.size);
        println!("Cree: {}", self.created_at.format("%Y-%m-%d %H:%M:%S"));
        println!("Modifie: {}", self.modified_at.format("%Y-%m-%d %H:%M:%S"));
        println!("===============================\n");
    }
}

fn main() {
    let mut file_manager = FileManager::new();
    let stdin = io::stdin();
    
    println!("=== GESTIONNAIRE DE FICHIERS RUST ===");
    println!("Commandes:");
    println!("create <nom> - Creer un fichier");
    println!("write <nom> - Ecrire dans un fichier");
    println!("read <nom> - Lire un fichier");
    println!("append <nom> - Ajouter du contenu");
    println!("delete <nom> - Supprimer un fichier");
    println!("info <nom> - Informations du fichier");
    println!("list - Lister les fichiers");
    println!("status - Statut du gestionnaire");
    println!("quit - Quitter");
    println!("=====================================\n");

    // Loop principal avec gestion des commandes
    loop {
        print!("Commande > ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                let command = input.trim();
                let parts: Vec<&str> = command.split_whitespace().collect();
                
                if parts.is_empty() {
                    continue;
                }

                // Match sur les différentes commandes
                match parts[0] {
                    "create" => {
                        if parts.len() < 2 {
                            println!("Usage: create <nom_fichier>");
                            continue;
                        }
                        let _ = file_manager.create_file(parts[1]);
                    }
                    
                    "write" => {
                        if parts.len() < 2 {
                            println!("Usage: write <nom_fichier>");
                            continue;
                        }
                        
                        print!("Contenu a ecrire: ");
                        io::stdout().flush().unwrap();
                        let mut content = String::new();
                        let _ = stdin.read_line(&mut content);
                        let _ = file_manager.write_file(parts[1], content.trim());
                    }
                    
                    "read" => {
                        if parts.len() < 2 {
                            println!("Usage: read <nom_fichier>");
                            continue;
                        }
                        let _ = file_manager.read_file(parts[1]);
                    }
                    
                    "append" => {
                        if parts.len() < 2 {
                            println!("Usage: append <nom_fichier>");
                            continue;
                        }
                        
                        print!("Contenu a ajouter: ");
                        io::stdout().flush().unwrap();
                        let mut content = String::new();
                        let _ = stdin.read_line(&mut content);
                        let _ = file_manager.append_to_file(parts[1], content.trim());
                    }
                    
                    "delete" => {
                        if parts.len() < 2 {
                            println!("Usage: delete <nom_fichier>");
                            continue;
                        }
                        
                        print!("Etes-vous sur de vouloir supprimer '{}'? (o/n): ", parts[1]);
                        io::stdout().flush().unwrap();
                        let mut confirmation = String::new();
                        let _ = stdin.read_line(&mut confirmation);
                        
                        match confirmation.trim().to_lowercase().as_str() {
                            "o" | "oui" | "y" | "yes" => {
                                let _ = file_manager.delete_file(parts[1]);
                            }
                            _ => println!("Suppression annulee"),
                        }
                    }
                    
                    "info" => {
                        if parts.len() < 2 {
                            println!("Usage: info <nom_fichier>");
                            continue;
                        }
                        
                        match file_manager.get_file_info(parts[1]) {
                            Ok(info) => info.display(),
                            Err(e) => println!("Erreur: {}", e),
                        }
                    }
                    
                    "list" => {
                        let files = file_manager.list_files();
                        if files.is_empty() {
                            println!("Aucun fichier dans le repertoire courant");
                        } else {
                            println!("Fichiers disponibles:");
                            let mut counter = 1;
                            while counter <= files.len() {
                                println!("  {}. {}", counter, files[counter - 1]);
                                counter += 1;
                            }
                        }
                    }
                    
                    "status" => {
                        file_manager.show_status();
                    }
                    
                    "quit" | "exit" | "q" => {
                        println!("Au revoir!");
                        break;
                    }
                    
                    _ => {
                        println!("Commande inconnue. Tapez 'quit' pour quitter.");
                    }
                }
            }
            Err(e) => {
                println!("Erreur de lecture: {}", e);
                break;
            }
        }
    }
}