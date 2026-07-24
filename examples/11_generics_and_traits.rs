// 11 - Generiques et traits
//
// Les generiques evitent de dupliquer du code pour chaque type. Les
// traits definissent un comportement partage (un peu comme une
// interface) que differents types peuvent implementer.
//
// Pour lancer cet exemple :
//   cargo run --example 11_generics_and_traits

// Fonction generique : T doit implementer PartialOrd (comparable) et
// Copy (pour pouvoir renvoyer une valeur sans deplacer les elements
// du slice). Ces contraintes s'appellent des "trait bounds".
fn plus_grand<T: PartialOrd + Copy>(elements: &[T]) -> T {
    let mut max = elements[0];
    for &element in elements.iter() {
        if element > max {
            max = element;
        }
    }
    max
}

// Struct generique sur deux types.
struct Paire<A, B> {
    premier: A,
    second: B,
}

impl<A: std::fmt::Display, B: std::fmt::Display> Paire<A, B> {
    fn afficher(&self) {
        println!("({}, {})", self.premier, self.second);
    }
}

// Definition d'un trait : un contrat que plusieurs types peuvent honorer.
trait Forme {
    fn aire(&self) -> f64;

    // Une methode par defaut : les types qui implementent Forme peuvent
    // la garder telle quelle ou la redefinir.
    fn decrire(&self) -> String {
        format!("une forme d'aire {:.2}", self.aire())
    }
}

struct Cercle {
    rayon: f64,
}

struct Carre {
    cote: f64,
}

impl Forme for Cercle {
    fn aire(&self) -> f64 {
        std::f64::consts::PI * self.rayon * self.rayon
    }
}

impl Forme for Carre {
    fn aire(&self) -> f64 {
        self.cote * self.cote
    }

    // Ici on redefinit le comportement par defaut.
    fn decrire(&self) -> String {
        format!("un carre de cote {} (aire {:.2})", self.cote, self.aire())
    }
}

// impl Trait en parametre : accepte n'importe quel type implementant Forme,
// sans avoir a ecrire de generique explicite <T: Forme>.
fn afficher_description(forme: &impl Forme) {
    println!("{}", forme.decrire());
}

// dyn Trait : permet de stocker des types differents dans une seule
// collection, tant qu'ils implementent tous le meme trait (dispatch
// dynamique, resolu a l'execution plutot qu'a la compilation).
fn aire_totale(formes: &[Box<dyn Forme>]) -> f64 {
    formes.iter().map(|forme| forme.aire()).sum()
}

fn main() {
    let entiers = vec![3, 7, 2, 9, 4];
    println!("plus grand entier : {}", plus_grand(&entiers));

    let flottants = vec![1.5, 2.8, 0.3];
    println!("plus grand flottant : {}", plus_grand(&flottants));

    let paire = Paire {
        premier: "Rust",
        second: 2026,
    };
    paire.afficher();

    let cercle = Cercle { rayon: 2.0 };
    let carre = Carre { cote: 3.0 };
    afficher_description(&cercle);
    afficher_description(&carre);

    let formes: Vec<Box<dyn Forme>> = vec![
        Box::new(Cercle { rayon: 1.0 }),
        Box::new(Carre { cote: 2.0 }),
    ];
    println!("aire totale des formes : {:.2}", aire_totale(&formes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_grand_fonctionne_sur_des_entiers() {
        assert_eq!(plus_grand(&[3, 7, 2, 9, 4]), 9);
    }

    #[test]
    fn plus_grand_fonctionne_sur_des_flottants() {
        assert_eq!(plus_grand(&[1.5, 2.8, 0.3]), 2.8);
    }

    #[test]
    fn methode_par_defaut_est_utilisee_si_non_redefinie() {
        let cercle = Cercle { rayon: 1.0 };
        assert!(cercle.decrire().contains("forme d'aire"));
    }

    #[test]
    fn methode_redefinie_change_le_comportement() {
        let carre = Carre { cote: 2.0 };
        assert!(carre.decrire().contains("carre de cote"));
    }
}
