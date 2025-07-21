use std::io::{self, Write};

struct Compte {
id: u32,
solde: f64,
}

impl Compte {
fn new(id: u32, solde_initial: f64) -> Self {
Compte { id, solde: solde_initial }
}


fn afficher(&self) {
    println!("- ID {} : Solde = {:.2} €", self.id, self.solde);
}

fn solde(&self) -> f64 {
    self.solde
}

fn retirer(&mut self, montant: f64) -> Result<f64, String> {
    if self.solde >= montant {
        self.solde -= montant;
        Ok(self.solde)
    } else {
        Err(format!("Solde insuffisant (solde actuel: {:.2} €)", self.solde))
    }
}


}

fn saisie_f64(prompt: &str) -> f64 {
print!("{}", prompt);
io::stdout().flush().unwrap();
let mut buf = String::new();
io::stdin().read_line(&mut buf).unwrap();
buf.trim().parse::<f64>().unwrap_or(0.0)
}

fn saisie_u32(prompt: &str) -> u32 {
print!("{}", prompt);
io::stdout().flush().unwrap();
let mut buf = String::new();
io::stdin().read_line(&mut buf).unwrap();
buf.trim().parse::<u32>().unwrap_or(0)
}

fn main() {
let mut comptes: Vec<Compte> = Vec::new();
let mut prochain_id = 1;


loop {
    println!("n=== MENU ===");
    println!("1. Créer un compte");
    println!("2. Afficher la liste des comptes");
    println!("3. Afficher le solde d’un compte");
    println!("4. Retirer de l’argent");
    println!("5. Quitter");

    let choix = saisie_u32("Choix : ");
    match choix {
        1 => {
            let solde = saisie_f64("Entrez le solde initial : ");
            let compte = Compte::new(prochain_id, solde);
            comptes.push(compte);
            println!("Compte créé avec l'ID {} et un solde de {:.2} €", prochain_id, solde);
            prochain_id += 1;
        }
        2 => {
            if comptes.is_empty() {
                println!("Aucun compte enregistré.");
            } else {
                println!("Liste des comptes :");
                for c in &comptes {
                    c.afficher();
                }
            }
        }
        3 => {
            let id = saisie_u32("Entrez l'ID du compte : ");
            if let Some(c) = comptes.iter().find(|c| c.id == id) {
                println!("Le solde du compte {} est {:.2} €", id, c.solde());
            } else {
                println!("Compte avec l'ID {} introuvable.", id);
            }
        }
        4 => {
            let id = saisie_u32("Entrez l'ID du compte : ");
            if let Some(c) = comptes.iter_mut().find(|c| c.id == id) {
                let montant = saisie_f64("Entrez le montant à retirer : ");
                match c.retirer(montant) {
                    Ok(nouveau_solde) => println!("Retrait de {:.2} € effectué. Nouveau solde : {:.2} €", montant, nouveau_solde),
                    Err(err) => println!("{}", err),
                }
            } else {
                println!("Compte avec l'ID {} introuvable.", id);
            }
        }
        5 => {
            println!("Au revoir !");
            break;
        }
        _ => println!("Choix invalide, veuillez réessayer."),
    }
}


}
