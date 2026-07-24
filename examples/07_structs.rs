// 07 - Structs et methodes
//
// Une struct regroupe des donnees liees sous un seul type. Les blocs
// impl ajoutent des methodes (qui prennent self) et des fonctions
// associees (qui n'en prennent pas, souvent des constructeurs).
//
// Pour lancer cet exemple :
//   cargo run --example 07_structs

#[derive(Debug)] // permet d'afficher la struct avec {:?}
struct Rectangle {
    largeur: f64,
    hauteur: f64,
}

impl Rectangle {
    // Fonction associee (pas de `self`) : convention pour un
    // constructeur, appelee via Rectangle::carre(5.0).
    fn carre(cote: f64) -> Rectangle {
        Rectangle {
            largeur: cote,
            hauteur: cote,
        }
    }

    // Methode avec emprunt immuable : lit les champs sans les modifier.
    fn aire(&self) -> f64 {
        self.largeur * self.hauteur
    }

    fn est_carre(&self) -> bool {
        self.largeur == self.hauteur
    }

    // Methode avec emprunt mutable : autorisee a modifier les champs.
    fn agrandir(&mut self, facteur: f64) {
        self.largeur *= facteur;
        self.hauteur *= facteur;
    }
}

// Tuple struct : utile quand les noms de champs n'apportent rien.
struct Point(f64, f64);

// Struct unitaire (sans champ) : sert surtout de marqueur pour les traits.
struct Marqueur;

fn main() {
    let mut rect = Rectangle {
        largeur: 30.0,
        hauteur: 10.0,
    };
    println!("rectangle : {:?}", rect);
    println!("aire = {}", rect.aire());
    println!("est un carre ? {}", rect.est_carre());

    rect.agrandir(2.0);
    println!("apres agrandissement : {:?}, aire = {}", rect, rect.aire());

    let carre = Rectangle::carre(5.0);
    println!("carre : {:?}, est un carre ? {}", carre, carre.est_carre());

    let origine = Point(0.0, 0.0);
    println!("point : ({}, {})", origine.0, origine.1);

    let _marqueur = Marqueur;

    // Syntaxe de mise a jour : cree une nouvelle instance en reprenant
    // les champs non precises depuis une autre instance existante.
    let autre_rect = Rectangle {
        largeur: 100.0,
        ..carre
    };
    println!("autre_rect : {:?}", autre_rect);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aire_se_calcule_correctement() {
        let rect = Rectangle {
            largeur: 4.0,
            hauteur: 5.0,
        };
        assert_eq!(rect.aire(), 20.0);
    }

    #[test]
    fn carre_construit_un_rectangle_a_cotes_egaux() {
        let carre = Rectangle::carre(3.0);
        assert!(carre.est_carre());
        assert_eq!(carre.aire(), 9.0);
    }

    #[test]
    fn agrandir_modifie_les_deux_dimensions() {
        let mut rect = Rectangle {
            largeur: 2.0,
            hauteur: 3.0,
        };
        rect.agrandir(2.0);
        assert_eq!(rect.largeur, 4.0);
        assert_eq!(rect.hauteur, 6.0);
    }
}
