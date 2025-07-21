use std::io::{self, Write};

struct Compte {
    id: u32,
    solde: f64,
}

fn main() {
    let mut comptes: Vec<Compte> = Vec::new();
    let mut prochain_id: u32 = 1;

    loop {
        println!("\n=== MENU ===");
        println!("1. Créer un compte");
        println!("2. Afficher la liste des comptes");
        println!("3. Afficher le solde d’un compte");
        println!("4. Retirer de l’argent");
        println!("5. Quitter");
        print!("Choix : ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        match choix.trim() {
            "1" => {
                // Création de compte
                print!("Entrez le solde initial : ");
                io::stdout().flush().unwrap();
                let mut solde_str = String::new();
                io::stdin().read_line(&mut solde_str).unwrap();
                let solde = solde_str.trim().parse::<f64>().unwrap_or(0.0);

                let compte = Compte { id: prochain_id, solde };
                comptes.push(compte);
                println!("Compte créé avec l'ID {} et un solde de {:.2} €", prochain_id, solde);
                prochain_id += 1;
            }
            "2" => {
                // Afficher tous les comptes
                if comptes.is_empty() {
                    println!("Aucun compte enregistré.");
                } else {
                    println!("Liste des comptes :");
                    for c in &comptes {
                        println!("- ID {} : Solde = {:.2} €", c.id, c.solde);
                    }
                }
            }
            "3" => {
                // Afficher solde d’un compte
                print!("Entrez l'ID du compte : ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id = id_str.trim().parse::<u32>().unwrap_or(0);

                match comptes.iter().find(|c| c.id == id) {
                    Some(c) => println!("Le solde du compte {} est {:.2} €", id, c.solde),
                    None => println!("Compte avec l'ID {} introuvable.", id),
                }
            }
            "4" => {
                // Retrait d’argent
                print!("Entrez l'ID du compte : ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id = id_str.trim().parse::<u32>().unwrap_or(0);

                if let Some(pos) = comptes.iter().position(|c| c.id == id) {
                    print!("Entrez le montant à retirer : ");
                    io::stdout().flush().unwrap();
                    let mut montant_str = String::new();
                    io::stdin().read_line(&mut montant_str).unwrap();
                    let montant = montant_str.trim().parse::<f64>().unwrap_or(0.0);

                    if comptes[pos].solde >= montant {
                        comptes[pos].solde -= montant;
                        println!("Retrait de {:.2} € effectué. Nouveau solde : {:.2} €", montant, comptes[pos].solde);
                    } else {
                        println!("Solde insuffisant pour un retrait de {:.2} € (solde actuel : {:.2} €)", montant, comptes[pos].solde);
                    }
                } else {
                    println!("Compte avec l'ID {} introuvable.", id);
                }
            }
            "5" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide, veuillez réessayer."),
        }
    }
}
