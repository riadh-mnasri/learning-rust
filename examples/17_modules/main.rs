// 17 - Modules et organisation multi-fichiers
//
// Jusqu'ici, chaque exemple tenait dans un seul fichier. Un vrai projet
// se découpe en modules : des unités qui regroupent du code lié et
// contrôlent ce qui est visible depuis l'extérieur. `mod nom;` charge
// le contenu depuis un fichier `nom.rs` (ou `nom/mod.rs`), tandis qu'un
// bloc `mod nom { ... }` définit un module directement en ligne.
//
// Pour lancer cet exemple :
//   cargo run --example 17_modules

// Chaque `mod` déclare un fichier associé : `mod catalogue;` charge
// `catalogue.rs`, `mod emprunts;` charge `emprunts.rs`. Sans cette
// déclaration à la racine, ces fichiers ne seraient jamais compilés.
mod catalogue;
mod emprunts;

// `use` importe un chemin pour éviter de le répéter en entier partout.
use catalogue::Livre;
use emprunts::Emprunt;

fn main() {
    let mut bibliotheque = catalogue::Bibliotheque::nouvelle();
    bibliotheque.ajouter(Livre::nouveau("Le Petit Prince", "Saint-Exupéry"));
    bibliotheque.ajouter(Livre::nouveau("1984", "Orwell"));
    bibliotheque.ajouter(Livre::nouveau("Fondation", "Asimov"));

    println!("catalogue : {} livres", bibliotheque.nombre_de_livres());
    for livre in bibliotheque.livres() {
        println!("  - {} ({})", livre.titre, livre.auteur);
    }

    // `stats` est un module imbriqué, défini directement dans
    // catalogue.rs plutôt que dans un fichier séparé : les deux
    // approches (fichier ou bloc en ligne) coexistent normalement.
    println!(
        "titre le plus long : {}",
        catalogue::stats::titre_le_plus_long(&bibliotheque).unwrap_or("aucun")
    );

    let emprunt = Emprunt::nouveau("Riadh", "1984");
    println!("{}", emprunt.decrire());

    // `emprunts::peut_emprunter` fait le pont entre les deux modules :
    // il appelle en interne `catalogue::interne::verifier_disponibilite`.
    // Cette dernière est `pub(crate)` : visible depuis n'importe où
    // dans ce binaire (donc depuis emprunts.rs, qui l'appelle ici, ou
    // directement depuis main.rs), mais pas depuis un autre crate qui
    // dépendrait de celui-ci.
    println!(
        "peut emprunter « 1984 » ? {}",
        emprunts::peut_emprunter(&bibliotheque, "1984")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bibliotheque_compte_bien_les_livres_ajoutes() {
        let mut bibliotheque = catalogue::Bibliotheque::nouvelle();
        bibliotheque.ajouter(Livre::nouveau("A", "Auteur A"));
        bibliotheque.ajouter(Livre::nouveau("B", "Auteur B"));
        assert_eq!(bibliotheque.nombre_de_livres(), 2);
    }

    #[test]
    fn emprunt_decrit_correctement() {
        let emprunt = Emprunt::nouveau("Alice", "Dune");
        assert_eq!(emprunt.decrire(), "Alice a emprunté « Dune »");
    }
}
