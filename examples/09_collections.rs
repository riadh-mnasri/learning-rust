// 09 - Collections courantes : Vec, String, HashMap
//
// Les trois collections de la bibliothèque standard qu'on utilise le
// plus au quotidien. Toutes allouent leurs données sur le tas et
// peuvent grandir dynamiquement, contrairement aux tableaux fixes.
//
// Pour lancer cet exemple :
//   cargo run --example 09_collections

// clippy::vec_init_then_push désactivé volontairement : Vec::new() puis
// push() est écrit explicitement pour montrer cette API avant
// d'utiliser la macro vec! juste après. clippy::useless_vec aussi, pour
// la même raison qu'ailleurs dans ce dépôt.
#![allow(clippy::vec_init_then_push, clippy::useless_vec)]

use std::collections::HashMap;

fn main() {
    // --- Vec<T> ---
    let mut notes: Vec<i32> = Vec::new();
    notes.push(12);
    notes.push(15);
    notes.push(9);
    println!("notes = {:?}", notes);

    // La macro vec! est un raccourci pour initialiser avec des valeurs.
    let notes = vec![12, 15, 9, 18];

    // accès sécurisé : get() renvoie Option<&T> au lieu de paniquer.
    match notes.get(10) {
        Some(note) => println!("note à l'index 10 : {}", note),
        None => println!("pas de note à l'index 10 (out of bounds)"),
    }

    let moyenne: f64 = notes.iter().sum::<i32>() as f64 / notes.len() as f64;
    println!("moyenne = {:.2}", moyenne);

    let notes_admises: Vec<&i32> = notes.iter().filter(|&&n| n >= 10).collect();
    println!("notes admises : {:?}", notes_admises);

    // --- String ---
    let mut phrase = String::from("Rust");
    phrase.push_str(" est");
    phrase.push(' ');
    phrase += "puissant";
    println!("phrase = {}", phrase);

    // Une String est garantie UTF-8 valide : on ne peut pas l'indexer
    // directement par position d'octet comme un tableau, car un
    // caractère peut occuper plusieurs octets. On itère sur les
    // caractères (chars) ou les octets (bytes) explicitement.
    let mot = String::from("café");
    println!("nombre de caractères : {}", mot.chars().count());
    for (i, c) in mot.chars().enumerate() {
        print!("[{}:{}] ", i, c);
    }
    println!();

    let mots: Vec<&str> = phrase.split(' ').collect();
    println!("mots découpés : {:?}", mots);

    // --- HashMap<K, V> ---
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(String::from("Riadh"), 33);
    ages.insert(String::from("Seji"), 11);
    ages.insert(String::from("Sany"), 8);

    match ages.get("Seji") {
        Some(age) => println!("Seji a {} ans", age),
        None => println!("personne inconnue"),
    }

    // entry().or_insert() : modifie si présent, insère sinon, en un
    // seul appel, sans double lookup.
    let compteur_ages = ages.entry(String::from("Syma")).or_insert(6);
    *compteur_ages += 0; // déjà la bonne valeur, juste pour illustrer l'accès mutable

    for (nom, age) in &ages {
        println!("{} -> {} ans", nom, age);
    }

    // Cas d'usage classique de entry : compter des occurrences.
    let mots_texte = "le chat et le chien et le chat";
    let mut occurrences: HashMap<&str, i32> = HashMap::new();
    for mot in mots_texte.split_whitespace() {
        *occurrences.entry(mot).or_insert(0) += 1;
    }
    println!("occurrences : {:?}", occurrences);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_hors_limites_renvoie_none() {
        let notes = vec![1, 2, 3];
        assert_eq!(notes.get(10), None);
    }

    #[test]
    fn filter_garde_seulement_les_notes_admises() {
        let notes = vec![8, 12, 5, 16];
        let admises: Vec<&i32> = notes.iter().filter(|&&n| n >= 10).collect();
        assert_eq!(admises, vec![&12, &16]);
    }

    #[test]
    fn entry_or_insert_compte_les_occurrences() {
        let mut occurrences: HashMap<&str, i32> = HashMap::new();
        for mot in "a b a c a b".split_whitespace() {
            *occurrences.entry(mot).or_insert(0) += 1;
        }
        assert_eq!(occurrences.get("a"), Some(&3));
        assert_eq!(occurrences.get("b"), Some(&2));
        assert_eq!(occurrences.get("c"), Some(&1));
    }
}
