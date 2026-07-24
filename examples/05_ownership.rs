// 05 - Ownership (possession)
//
// C'est LA notion centrale de Rust : chaque valeur a un unique
// proprietaire, et quand ce proprietaire sort de portee, la valeur est
// liberee automatiquement (drop), sans garbage collector.
//
// Pour lancer cet exemple :
//   cargo run --example 05_ownership

fn main() {
    // Les types simples (entiers, bool, char...) implementent le trait
    // Copy : les affecter fait une copie bit a bit, pas un transfert
    // de possession. Aucune des deux variables n'est invalidee.
    let x = 5;
    let y = x;
    println!("x = {}, y = {} (les deux sont valides, i32 implemente Copy)", x, y);

    // String, en revanche, possede des donnees allouees sur le tas.
    // Affecter s1 a s2 deplace (move) la possession : s1 n'est plus
    // utilisable ensuite. Decommentez la ligne suivante pour voir
    // l'erreur de compilation "value borrowed after move".
    let s1 = String::from("bonjour");
    let s2 = s1;
    // println!("{}", s1); // erreur de compilation si decommente
    println!("s2 = {}", s2);

    // Pour obtenir une vraie copie independante d'un String, on clone
    // explicitement (cout memoire assume, contrairement au move).
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {} (deux String independantes)", s2, s3);

    // Passer une valeur a une fonction en deplace aussi la possession,
    // sauf si le type implemente Copy.
    let message = String::from("consomme-moi");
    consommer(message);
    // println!("{}", message); // erreur : message a ete deplace dans consommer()

    let nombre = 42;
    consommer_copie(nombre);
    println!("nombre = {} (toujours valide, i32 est Copy)", nombre);

    // Une fonction peut aussi rendre la possession en retournant la
    // valeur, ce qui permet de continuer a l'utiliser ensuite.
    let recupere = String::from("valeur");
    let recupere = redonner_la_possession(recupere);
    println!("recupere apres aller-retour : {}", recupere);

    // La portee (scope) determine quand une valeur est detruite (drop).
    {
        let temporaire = String::from("je vis dans ce bloc");
        println!("dans le bloc : {}", temporaire);
    } // temporaire est droppee ici, sa memoire est liberee
    println!("hors du bloc, temporaire n'existe plus");
}

fn consommer(texte: String) {
    println!("fonction consommer a recu : {}", texte);
    // texte est droppe a la fin de cette fonction
}

fn consommer_copie(nombre: i32) {
    println!("fonction consommer_copie a recu une copie : {}", nombre);
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
