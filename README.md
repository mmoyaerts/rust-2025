# Résumé rapide de Rust

* **Fonctions** : `fn nom(params) -> type { ... }`

  ```rust
  fn add(a: i32, b: i32) -> i32 { a + b }
  ```

* **Variables** :

  * Immuables : `let x = 5;`
  * Mutables : `let mut y = 10; y = 15;`

* **Affichage** : `println!("{}", valeur);`

* **Vecteurs** : `let mut v = Vec::new(); v.push(1);`

* **Références** :

  * Immuable : `&T` (lecture)
  * Mutable : `&mut T` (lecture/écriture)
