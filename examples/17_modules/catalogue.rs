// Module `catalogue` : chargé depuis main.rs via `mod catalogue;`.
// Tout ce qui doit être visible depuis l'extérieur du module (main.rs,
// emprunts.rs...) doit être marqué `pub` explicitement : par défaut,
// tout est privé au module qui le définit.

// Livre expose ses champs directement (`pub` sur chaque champ) :
// n'importe qui peut les lire ou les modifier. À l'inverse,
// Bibliotheque ci-dessous garde son champ privé et n'expose que des
// méthodes : deux façons légitimes de concevoir une API, selon qu'on
// veut ou non garder le contrôle sur les invariants internes.
pub struct Livre {
    pub titre: String,
    pub auteur: String,
}

impl Livre {
    pub fn nouveau(titre: &str, auteur: &str) -> Livre {
        Livre {
            titre: titre.to_string(),
            auteur: auteur.to_string(),
        }
    }
}

pub struct Bibliotheque {
    // Ce champ n'est pas `pub` : il reste privé au module `catalogue`
    // et à ses sous-modules (comme `stats` ci-dessous), mais reste
    // inaccessible depuis main.rs ou emprunts.rs.
    livres: Vec<Livre>,
}

impl Bibliotheque {
    pub fn nouvelle() -> Bibliotheque {
        Bibliotheque { livres: Vec::new() }
    }

    pub fn ajouter(&mut self, livre: Livre) {
        self.livres.push(livre);
    }

    pub fn nombre_de_livres(&self) -> usize {
        self.livres.len()
    }

    pub fn livres(&self) -> &[Livre] {
        &self.livres
    }
}

// Module imbriqué, défini en ligne plutôt que dans un fichier séparé :
// les deux formes (`mod nom;` + fichier, ou `mod nom { ... }` en ligne)
// s'utilisent normalement dans un même projet, selon la taille du
// contenu. Un sous-module hérite de la visibilité de son parent : il
// peut donc lire le champ privé `livres` défini juste au-dessus.
pub mod stats {
    use super::Bibliotheque;

    pub fn titre_le_plus_long(bibliotheque: &Bibliotheque) -> Option<&str> {
        bibliotheque
            .livres
            .iter()
            .max_by_key(|livre| livre.titre.len())
            .map(|livre| livre.titre.as_str())
    }
}

// `pub(crate)` rend ce module visible partout dans ce binaire (donc
// depuis emprunts.rs), mais pas depuis un crate externe qui
// dépendrait de celui-ci, contrairement à `pub`.
pub(crate) mod interne {
    use super::Bibliotheque;

    pub(crate) fn verifier_disponibilite(bibliotheque: &Bibliotheque, titre: &str) -> bool {
        bibliotheque.livres.iter().any(|livre| livre.titre == titre)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titre_le_plus_long_trouve_le_bon_livre() {
        let mut bibliotheque = Bibliotheque::nouvelle();
        bibliotheque.ajouter(Livre::nouveau("Court", "Auteur A"));
        bibliotheque.ajouter(Livre::nouveau("Un titre bien plus long", "Auteur B"));

        assert_eq!(
            stats::titre_le_plus_long(&bibliotheque),
            Some("Un titre bien plus long")
        );
    }

    #[test]
    fn titre_le_plus_long_renvoie_none_si_vide() {
        let bibliotheque = Bibliotheque::nouvelle();
        assert_eq!(stats::titre_le_plus_long(&bibliotheque), None);
    }

    #[test]
    fn verifier_disponibilite_detecte_un_livre_present() {
        let mut bibliotheque = Bibliotheque::nouvelle();
        bibliotheque.ajouter(Livre::nouveau("Dune", "Herbert"));
        assert!(interne::verifier_disponibilite(&bibliotheque, "Dune"));
        assert!(!interne::verifier_disponibilite(&bibliotheque, "Inconnu"));
    }
}
