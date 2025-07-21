# README

Petit programme bancaire en Rust gérant plusieurs comptes.

## Fonctionnalités

* **Création de compte** avec :

  * Identifiant auto-incrémenté
  * Nom du titulaire
  * Solde initial

* **Affichage** de tous les comptes ou d’un compte spécifique

* **Consultation** du solde d’un compte

* **Dépôt** d’un montant (interdit si montant négatif)

* **Retrait** d’un montant (interdit si montant négatif ou solde insuffisant)

* **Renommage** d’un compte (retourne un nouveau compte avec le nouveau nom)

## Organisation du code

* **Struct `CompteBancaire`**

  * Champs : `id: u32`, `nom: String`, `solde: f64`
  * Méthodes :

    * `new(id, nom, solde_initial)`
    * `afficher()`
    * `solde() -> f64`
    * `deposer(montant) -> Result<f64, String>`
    * `retirer(montant) -> Result<f64, String>`
    * `renommer(nouveau_nom) -> CompteBancaire`

* **Fonctions utilitaires de saisie**

  * `saisie_string(prompt) -> String`
  * `saisie_f64(prompt) -> f64`
  * `saisie_u32(prompt) -> u32`

* **Boucle principale (`main`)**

  * Menu en boucle avec choix 1 à 7
  * Gestion des cas selon le choix
  * Parcours du `Vec<CompteBancaire>` avec `iter()` et `enumerate()`

## Utilisation

```bash
cargo run
```

