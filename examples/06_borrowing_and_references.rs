// 06 - Emprunts et références
//
// Emprunter une valeur (&) permet de l'utiliser sans en prendre
// possession. Règle d'or vérifiée par le compilateur (le "borrow
// checker") : à un instant donné, soit plusieurs références
// immuables, soit une seule référence mutable, jamais les deux.
//
// Pour lancer cet exemple :
//   cargo run --example 06_borrowing_and_references

// clippy::ptr_arg désactivé volontairement : calculer_longueur prend
// &String plutôt que &str pour illustrer l'emprunt d'un String avant
// d'introduire &str juste après. clippy::useless_vec désactivé aussi,
// vec! est utilisé ici pour montrer sa syntaxe, pas parce qu'un tableau
// serait insuffisant.
#![allow(clippy::ptr_arg, clippy::useless_vec)]

fn main() {
    let texte = String::from("bonjour le monde");

    // & crée une référence immuable : on prête la valeur sans la
    // déplacer, `texte` reste utilisable après l'appel.
    let longueur = calculer_longueur(&texte);
    println!("'{}' fait {} caractères", texte, longueur);

    // &mut permet de modifier la valeur empruntée.
    let mut modifiable = String::from("bonjour");
    ajouter_suffixe(&mut modifiable);
    println!("après modification : {}", modifiable);

    // Plusieurs références immuables simultanées : autorisé.
    let a = &modifiable;
    let b = &modifiable;
    println!("deux emprunts immuables en même temps : {} / {}", a, b);

    // En revanche, une référence mutable exclut toute autre référence
    // (mutable ou non) tant qu'elle est utilisée. Le code suivant ne
    // compile pas si on décommente les deux lignes ensemble :
    // let ref_mut = &mut modifiable;
    // println!("{}", a); // erreur : `a` et `ref_mut` ne peuvent pas coexister

    // Slices : une référence vers une partie contiguë d'une collection,
    // sans copier les données.
    let phrase = String::from("Rust est agréable à apprendre");
    let premier_mot = extraire_premier_mot(&phrase);
    println!("premier mot : {}", premier_mot);

    let nombres = vec![10, 20, 30, 40, 50];
    let extrait: &[i32] = &nombres[1..4];
    println!("slice de nombres : {:?}", extrait);
}

// Prendre `&String` en paramètre signifie "j'emprunte, je ne prends pas
// possession". Appeler cette fonction ne consomme donc pas `texte`.
fn calculer_longueur(s: &String) -> usize {
    s.len()
}

fn ajouter_suffixe(s: &mut String) {
    s.push_str(" (modifié)");
}

// &str est une slice de chaîne : plus générique que &String, elle
// fonctionne aussi bien sur des String que sur des littéraux "...".
fn extraire_premier_mot(s: &str) -> &str {
    match s.find(' ') {
        Some(index) => &s[..index],
        None => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculer_longueur_ne_consomme_pas_la_chaine() {
        let texte = String::from("abcd");
        let longueur = calculer_longueur(&texte);
        assert_eq!(longueur, 4);
        // `texte` est toujours utilisable ici : preuve que c'est un emprunt.
        assert_eq!(texte, "abcd");
    }

    #[test]
    fn extraire_premier_mot_sans_espace_renvoie_tout() {
        assert_eq!(extraire_premier_mot("solo"), "solo");
    }

    #[test]
    fn extraire_premier_mot_avec_espace() {
        assert_eq!(extraire_premier_mot("bonjour le monde"), "bonjour");
    }
}
