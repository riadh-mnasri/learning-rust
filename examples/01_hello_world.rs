// 01 - Hello world
//
// Premier programme Rust. Objectif : comprendre la structure minimale
// d'un binaire, la macro println! et la difference entre un commentaire
// de ligne (//) et un commentaire de documentation (///, //!).
//
// Pour lancer cet exemple :
//   cargo run --example 01_hello_world

fn main() {
    // println! est une macro (le ! le signale), pas une fonction classique.
    // Elle gere le formatage et l'ecriture sur la sortie standard.
    println!("Hello, Rust !");

    // Les accolades {} sont des emplacements de substitution.
    let langage = "Rust";
    let annee_apprentissage = 2026;
    println!("J'apprends {} en {}.", langage, annee_apprentissage);

    // On peut nommer les emplacements pour plus de lisibilite.
    println!(
        "{nom} a ete cree pour la securite memoire sans garbage collector.",
        nom = langage
    );

    // eprintln! ecrit sur la sortie d'erreur, utile pour separer logs et
    // resultats dans un vrai programme.
    eprintln!("(ceci est un message de log, pas un resultat)");

    // format! construit une String sans rien afficher, tres utilise
    // des qu'on veut assembler du texte au lieu de l'imprimer directement.
    let message = format!("{} exemples pour apprendre {}", 16, langage);
    println!("{}", message);
}

#[cfg(test)]
mod tests {
    // On peut tester la logique de formatage meme dans un simple exemple.
    #[test]
    fn format_construit_la_chaine_attendue() {
        let message = format!("{} exemples pour apprendre {}", 16, "Rust");
        assert_eq!(message, "16 exemples pour apprendre Rust");
    }
}
