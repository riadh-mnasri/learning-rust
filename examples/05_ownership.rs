// 05 - Ownership (possession)
//
// C'est LA notion centrale de Rust : chaque valeur a un unique
// propriétaire, et quand ce propriétaire sort de portée, la valeur est
// libérée automatiquement (drop), sans garbage collector.
//
// Pour lancer cet exemple :
//   cargo run --example 05_ownership

fn main() {
    // Les types simples (entiers, bool, char...) implémentent le trait
    // Copy : les affecter fait une copie bit à bit, pas un transfert
    // de possession. Aucune des deux variables n'est invalidée.
    let x = 5;
    let y = x;
    println!("x = {}, y = {} (les deux sont valides, i32 implémente Copy)", x, y);

    // String, en revanche, possède des données allouées sur le tas.
    // Affecter s1 à s2 déplace (move) la possession : s1 n'est plus
    // utilisable ensuite. Décommentez la ligne suivante pour voir
    // l'erreur de compilation "value borrowed after move".
    let s1 = String::from("bonjour");
    let s2 = s1;
    // println!("{}", s1); // erreur de compilation si décommenté
    println!("s2 = {}", s2);

    // Pour obtenir une vraie copie indépendante d'un String, on clone
    // explicitement (coût mémoire assumé, contrairement au move).
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {} (deux String indépendantes)", s2, s3);

    // Passer une valeur à une fonction en déplace aussi la possession,
    // sauf si le type implémente Copy.
    let message = String::from("consomme-moi");
    consommer(message);
    // println!("{}", message); // erreur : message a été déplacé dans consommer()

    let nombre = 42;
    consommer_copie(nombre);
    println!("nombre = {} (toujours valide, i32 est Copy)", nombre);

    // Une fonction peut aussi rendre la possession en retournant la
    // valeur, ce qui permet de continuer à l'utiliser ensuite.
    let recupere = String::from("valeur");
    let recupere = redonner_la_possession(recupere);
    println!("recupere après aller-retour : {}", recupere);

    // La portée (scope) détermine quand une valeur est détruite (drop).
    {
        let temporaire = String::from("je vis dans ce bloc");
        println!("dans le bloc : {}", temporaire);
    } // temporaire est droppée ici, sa mémoire est libérée
    println!("hors du bloc, temporaire n'existe plus");
}

fn consommer(texte: String) {
    println!("fonction consommer a reçu : {}", texte);
    // texte est droppé à la fin de cette fonction
}

fn consommer_copie(nombre: i32) {
    println!("fonction consommer_copie a reçu une copie : {}", nombre);
}

fn redonner_la_possession(texte: String) -> String {
    println!("on transforme puis on rend : {}", texte);
    texte
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_produit_une_valeur_independante() {
        let original = String::from("abc");
        let copie = original.clone();
        assert_eq!(original, copie);
    }

    #[test]
    fn redonner_la_possession_renvoie_la_meme_valeur() {
        let valeur = String::from("test");
        let valeur = redonner_la_possession(valeur);
        assert_eq!(valeur, "test");
    }
}
