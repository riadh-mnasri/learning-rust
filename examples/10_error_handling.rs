// 10 - Gestion des erreurs : Option, Result, panic!, l'opérateur ?
//
// Rust distingue deux familles d'erreurs : les erreurs irrécupérables
// (panic!, on arrête le programme) et les erreurs récupérables
// (Result<T, E>, on force l'appelant à les traiter explicitement).
//
// Pour lancer cet exemple :
//   cargo run --example 10_error_handling

use std::fmt;

// Un type d'erreur maison, simple mais explicite. En vrai projet on
// utiliserait souvent le crate `thiserror`, mais l'implémenter à la
// main une fois aide à comprendre ce qu'il automatise.
#[derive(Debug, PartialEq)]
enum ErreurAge {
    Negatif(i32),
    TropGrand(i32),
}

impl fmt::Display for ErreurAge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErreurAge::Negatif(valeur) => write!(f, "l'âge ne peut pas être négatif : {}", valeur),
            ErreurAge::TropGrand(valeur) => write!(f, "âge improbable : {}", valeur),
        }
    }
}

fn valider_age(age: i32) -> Result<u32, ErreurAge> {
    if age < 0 {
        Err(ErreurAge::Negatif(age))
    } else if age > 150 {
        Err(ErreurAge::TropGrand(age))
    } else {
        Ok(age as u32)
    }
}

// L'opérateur ? propage l'erreur automatiquement : si l'appel échoue,
// la fonction retourne immédiatement avec ce Err. Sinon elle continue
// avec la valeur déballée. Cela évite d'empiler des match imbriqués.
fn calculer_annee_naissance(age: i32, annee_actuelle: i32) -> Result<i32, ErreurAge> {
    let age_valide = valider_age(age)?;
    Ok(annee_actuelle - age_valide as i32)
}

fn main() {
    // --- Option : absence de valeur, pas d'erreur à proprement parler ---
    let mots = vec!["12", "abc", "7"];
    for mot in &mots {
        // parse() renvoie un Result ; .ok() le convertit en Option en
        // jetant l'information d'erreur quand elle ne nous intéresse pas.
        let nombre: Option<i32> = mot.parse().ok();
        println!("'{}' -> {:?}", mot, nombre);
    }

    // --- Result : succès ou échec explicite ---
    for age in [25, -3, 200] {
        match valider_age(age) {
            Ok(age_ok) => println!("age {} valide", age_ok),
            Err(e) => println!("age {} invalide : {}", age, e),
        }
    }

    // Utilisation de l'opérateur ? via une fonction intermédiaire.
    match calculer_annee_naissance(33, 2026) {
        Ok(annee) => println!("année de naissance estimée : {}", annee),
        Err(e) => println!("impossible de calculer : {}", e),
    }
    match calculer_annee_naissance(-1, 2026) {
        Ok(annee) => println!("année de naissance estimée : {}", annee),
        Err(e) => println!("impossible de calculer : {}", e),
    }

    // unwrap_or et unwrap_or_else : fournir une valeur par défaut plutôt
    // que de paniquer, quand une erreur n'est pas bloquante.
    let age_par_defaut = valider_age(-10).unwrap_or(0);
    println!("age avec valeur par défaut : {}", age_par_defaut);

    // panic! est réservé aux bugs / états vraiment impossibles, pas à la
    // validation normale des entrées utilisateur. On ne l'appelle pas
    // ici pour ne pas interrompre l'exemple, mais voici la forme :
    // panic!("état impossible atteint");

    // unwrap()/expect() paniquent si la valeur est Err/None : pratiques
    // en prototype ou en test, à éviter en code de production sur des
    // entrées non maîtrisées.
    let toujours_valide = valider_age(20).expect("20 doit toujours être valide");
    println!("toujours_valide = {}", toujours_valide);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn age_negatif_est_refuse() {
        assert!(matches!(valider_age(-1), Err(ErreurAge::Negatif(-1))));
    }

    #[test]
    fn age_trop_grand_est_refuse() {
        assert!(matches!(valider_age(200), Err(ErreurAge::TropGrand(200))));
    }

    #[test]
    fn age_valide_est_accepte() {
        assert_eq!(valider_age(30), Ok(30));
    }

    #[test]
    fn calculer_annee_naissance_propage_les_erreurs_avec_interrogation() {
        assert!(calculer_annee_naissance(-1, 2026).is_err());
        assert_eq!(calculer_annee_naissance(26, 2026), Ok(2000));
    }
}
