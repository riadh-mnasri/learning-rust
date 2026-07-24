// 08 - Enums et pattern matching
//
// Un enum represente "une valeur parmi plusieurs variantes possibles",
// chaque variante pouvant transporter ses propres donnees. Combine a
// match, c'est l'outil principal pour modeliser des etats en Rust.
//
// Pour lancer cet exemple :
//   cargo run --example 08_enums_and_pattern_matching

// clippy::redundant_pattern_matching desactive volontairement : `if let
// None = rien` illustre la syntaxe if let sur la variante None avant
// d'utiliser is_none() ailleurs. clippy::manual_range_patterns aussi :
// le pattern `1 | 2 | 3 | 4 | 5` montre l'operateur | avant d'introduire
// les plages `1..=5` dans l'exemple precedent (04_control_flow).
#![allow(
    clippy::redundant_pattern_matching,
    clippy::manual_range_patterns
)]

#[derive(Debug)]
enum StatutCommande {
    EnAttente,
    Expediee { transporteur: String, numero_suivi: String },
    Livree(u32), // nombre de jours de livraison
    Annulee { raison: String },
}

fn decrire_statut(statut: &StatutCommande) -> String {
    // match doit couvrir tous les cas possibles : le compilateur refuse
    // de compiler si une variante est oubliee. C'est ce qui rend les
    // enums si surs pour modeliser des etats metier.
    match statut {
        StatutCommande::EnAttente => String::from("en attente de traitement"),
        StatutCommande::Expediee {
            transporteur,
            numero_suivi,
        } => format!("expediee par {} (suivi : {})", transporteur, numero_suivi),
        StatutCommande::Livree(jours) => format!("livree en {} jours", jours),
        StatutCommande::Annulee { raison } => format!("annulee : {}", raison),
    }
}

fn main() {
    let commandes = vec![
        StatutCommande::EnAttente,
        StatutCommande::Expediee {
            transporteur: String::from("Colissimo"),
            numero_suivi: String::from("6X4F2A"),
        },
        StatutCommande::Livree(3),
        StatutCommande::Annulee {
            raison: String::from("rupture de stock"),
        },
    ];

    for commande in &commandes {
        println!("{}", decrire_statut(commande));
    }

    // Option<T> est un enum de la bibliotheque standard : Some(valeur)
    // ou None. Il remplace l'usage de null, absent du langage Rust.
    let peut_etre_un_nombre: Option<i32> = Some(7);
    let rien: Option<i32> = None;

    match peut_etre_un_nombre {
        Some(n) => println!("valeur presente : {}", n),
        None => println!("aucune valeur"),
    }

    // if let : raccourci pratique quand on ne s'interesse qu'a un seul cas.
    if let Some(n) = peut_etre_un_nombre {
        println!("(if let) valeur presente : {}", n);
    }
    if let None = rien {
        println!("(if let) bien aucune valeur");
    }

    // match avec des gardes (conditions supplementaires) et des plages.
    let note = 13;
    let appreciation = match note {
        0..=9 => "insuffisant",
        10..=13 => "passable",
        14..=15 => "bien",
        n if n >= 16 => "tres bien",
        _ => "note invalide",
    };
    println!("note {} -> {}", note, appreciation);

    // Combiner plusieurs valeurs avec | dans un seul bras de match.
    let jour = 6;
    match jour {
        1 | 2 | 3 | 4 | 5 => println!("jour {} : jour ouvre", jour),
        6 | 7 => println!("jour {} : week-end", jour),
        _ => println!("jour invalide"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrire_statut_en_attente() {
        assert_eq!(
            decrire_statut(&StatutCommande::EnAttente),
            "en attente de traitement"
        );
    }

    #[test]
    fn decrire_statut_livree() {
        assert_eq!(decrire_statut(&StatutCommande::Livree(2)), "livree en 2 jours");
    }

    #[test]
    fn option_some_et_none_se_distinguent() {
        let valeur: Option<i32> = Some(5);
        let absente: Option<i32> = None;
        assert!(valeur.is_some());
        assert!(absente.is_none());
    }
}
