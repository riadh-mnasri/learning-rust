// Module `emprunts` : chargé depuis main.rs via `mod emprunts;`.
// Ce module ne connaît pas le module `catalogue` par défaut : il faut
// un `use` explicite pour accéder à ce qu'il expose en `pub`.

pub struct Emprunt {
    emprunteur: String,
    titre_livre: String,
}

impl Emprunt {
    pub fn nouveau(emprunteur: &str, titre_livre: &str) -> Emprunt {
        Emprunt {
            emprunteur: emprunteur.to_string(),
            titre_livre: titre_livre.to_string(),
        }
    }

    pub fn decrire(&self) -> String {
        format!("{} a emprunté « {} »", self.emprunteur, self.titre_livre)
    }
}

// `crate::catalogue::interne` est `pub(crate)` (voir catalogue.rs) :
// ce module peut donc l'appeler, même s'il ne serait pas visible
// depuis un autre crate qui dépendrait de ce binaire.
pub fn peut_emprunter(bibliotheque: &crate::catalogue::Bibliotheque, titre: &str) -> bool {
    crate::catalogue::interne::verifier_disponibilite(bibliotheque, titre)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalogue::{Bibliotheque, Livre};

    #[test]
    fn peut_emprunter_verifie_via_le_module_interne_du_catalogue() {
        let mut bibliotheque = Bibliotheque::nouvelle();
        bibliotheque.ajouter(Livre::nouveau("Dune", "Herbert"));

        assert!(peut_emprunter(&bibliotheque, "Dune"));
        assert!(!peut_emprunter(&bibliotheque, "Fondation"));
    }
}
