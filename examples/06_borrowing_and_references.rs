// 06 - Emprunts et references
//
// Emprunter une valeur (&) permet de l'utiliser sans en prendre
// possession. Regle d'or verifiee par le compilateur (le "borrow
// checker") : a un instant donne, soit plusieurs references
// immuables, soit une seule reference mutable, jamais les deux.
//
// Pour lancer cet exemple :
//   cargo run --example 06_borrowing_and_references

// clippy::ptr_arg desactive volontairement : calculer_longueur prend
// &String plutot que &str pour illustrer l'emprunt d'un String avant
// d'introduire &str juste apres. clippy::useless_vec desactive aussi,
// vec! est utilise ici pour montrer sa syntaxe, pas parce qu'un tableau
// serait insuffisant.
#![allow(clippy::ptr_arg, clippy::useless_vec)]

fn main() {
    let texte = String::from("bonjour le monde");

    // & cree une reference immuable : on prete la valeur sans la
    // deplacer, `texte` reste utilisable apres l'appel.
    let longueur = calculer_longueur(&texte);
    println!("'{}' fait {} caracteres", texte, longueur);

    // &mut permet de modifier la valeur empruntee.
    let mut modifiable = String::from("bonjour");
    ajouter_suffixe(&mut modifiable);
    println!("apres modification : {}", modifiable);

    // Plusieurs references immuables simultanees : autorise.
    let a = &modifiable;
    let b = &modifiable;
    println!("deux emprunts immuables en meme temps : {} / {}", a, b);

    // En revanche, une reference mutable exclut toute autre reference
    // (mutable ou non) tant qu'elle est utilisee. Le code suivant ne
    // compile pas si on decommente les deux lignes ensemble :
    // let ref_mut = &mut modifiable;
    // println!("{}", a); // erreur : `a` et `ref_mut` ne peuvent pas coexister

    // Slices : une reference vers une partie contigue d'une collection,
    // sans copier les donnees.
    let phrase = String::from("Rust est agreable a apprendre");
    let premier_mot = extraire_premier_mot(&phrase);
    println!("premier mot : {}", premier_mot);

    let nombres = vec![10, 20, 30, 40, 50];
    let extrait: &[i32] = &nombres[1..4];
    println!("slice de nombres : {:?}", extrait);
}

// Prendre `&String` en parametre signifie "j'emprunte, je ne prends pas
// possession". Appeler cette fonction ne consomme donc pas `texte`.
fn calculer_longueur(s: &String) -> usize {
    s.len()
}

fn ajouter_suffixe(s: &mut String) {
    s.push_str(" (modifie)");
}

// &str est une slice de chaine : plus generique que &String, elle
// fonctionne aussi bien sur des String que sur des litteraux "...".
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
