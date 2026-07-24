// 01 - Hello world
//
// Premier programme Rust. Objectif : comprendre la structure minimale
// d'un binaire, la macro println! et la différence entre un commentaire
// de ligne (//) et un commentaire de documentation (///, //!).
//
// Pour lancer cet exemple :
//   cargo run --example 01_hello_world

fn main() {
    // println! est une macro (le ! le signale), pas une fonction classique.
    // Elle gère le formatage et l'écriture sur la sortie standard.
    println!("Hello, Rust !");

    // Les accolades {} sont des emplacements de substitution.
    let langage = "Rust";
    let annee_apprentissage = 2026;
    println!("J'apprends {} en {}.", langage, annee_apprentissage);

    // On peut nommer les emplacements pour plus de lisibilité.
    println!(
        "{nom} a été créé pour la sécurité mémoire sans garbage collector.",
        nom = langage
    );

    // eprintln! écrit sur la sortie d'erreur, utile pour séparer logs et
    // résultats dans un vrai programme.
    eprintln!("(ceci est un message de log, pas un résultat)");

    // format! construit une String sans rien afficher, très utilisé
    // dès qu'on veut assembler du texte au lieu de l'imprimer directement.
    let message = format!("{} exemples pour apprendre {}", 16, langage);
    println!("{}", message);
}

#[cfg(test)]
mod tests {
    // On peut tester la logique de formatage même dans un simple exemple.
    #[test]
    fn format_construit_la_chaine_attendue() {
        let message = format!("{} exemples pour apprendre {}", 16, "Rust");
        assert_eq!(message, "16 exemples pour apprendre Rust");
    }
}
