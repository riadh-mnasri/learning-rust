// 14 - Smart pointers : Box, Rc, RefCell
//
// Ces types encapsulent une valeur et ajoutent une capacite : Box
// alloue sur le tas, Rc permet plusieurs proprietaires en lecture
// seule, RefCell autorise la mutation controlee a l'execution plutot
// qu'a la compilation.
//
// Pour lancer cet exemple :
//   cargo run --example 14_smart_pointers

use std::cell::RefCell;
use std::rc::Rc;

// Box<T> est indispensable pour les types recursifs : sans lui, le
// compilateur ne peut pas connaitre la taille de Liste (elle contient
// potentiellement une infinite d'elements imbriques). Box place les
// donnees sur le tas et ne stocke qu'un pointeur de taille fixe.
#[derive(Debug)]
enum Liste {
    Element(i32, Box<Liste>),
    Fin,
}

use Liste::{Element, Fin};

fn somme_liste(liste: &Liste) -> i32 {
    match liste {
        Element(valeur, suite) => valeur + somme_liste(suite),
        Fin => 0,
    }
}

// Rc<T> (Reference Counted) permet a plusieurs parties du programme de
// partager la possession d'une meme valeur en lecture. Le compteur de
// references augmente avec clone() et diminue quand un Rc est droppe ;
// la valeur n'est liberee que quand ce compteur atteint zero.
#[derive(Debug)]
struct Configuration {
    nom_application: String,
}

fn main() {
    let liste = Element(1, Box::new(Element(2, Box::new(Element(3, Box::new(Fin))))));
    println!("liste : {:?}", liste);
    println!("somme de la liste : {}", somme_liste(&liste));

    let config = Rc::new(Configuration {
        nom_application: String::from("learning-rust"),
    });
    println!("compteur de references initial : {}", Rc::strong_count(&config));

    let config_module_a = Rc::clone(&config);
    let config_module_b = Rc::clone(&config);
    println!(
        "apres deux clones, compteur = {}",
        Rc::strong_count(&config)
    );
    println!(
        "module A voit : {}, module B voit : {}",
        config_module_a.nom_application, config_module_b.nom_application
    );

    drop(config_module_a);
    println!(
        "apres un drop, compteur = {}",
        Rc::strong_count(&config)
    );

    // RefCell<T> deplace la verification des regles d'emprunt (une
    // reference mutable exclusive OU plusieurs immuables) de la
    // compilation vers l'execution. Cela permet de muter une valeur
    // meme quand on n'y a qu'un acces immuable en apparence (le
    // "mutable borrow interieur", ou "interior mutability").
    let compteur_visites = RefCell::new(0);
    {
        let mut acces_mutable = compteur_visites.borrow_mut();
        *acces_mutable += 1;
    } // l'emprunt mutable se termine ici, avant le suivant

    {
        let mut acces_mutable = compteur_visites.borrow_mut();
        *acces_mutable += 1;
    }

    println!("compteur de visites = {}", compteur_visites.borrow());

    // Rc<RefCell<T>> combine les deux : plusieurs proprietaires qui
    // peuvent chacun modifier la valeur partagee. Tres courant pour
    // representer un etat partage et mutable sans thread (voir
    // l'exemple 15 pour l'equivalent thread-safe avec Arc<Mutex<T>>).
    let etat_partage = Rc::new(RefCell::new(vec![String::from("premiere entree")]));
    let etat_pour_a = Rc::clone(&etat_partage);
    let etat_pour_b = Rc::clone(&etat_partage);

    etat_pour_a.borrow_mut().push(String::from("ajoute par a"));
    etat_pour_b.borrow_mut().push(String::from("ajoute par b"));

    println!("etat partage final : {:?}", etat_partage.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn somme_liste_additionne_tous_les_elements() {
        let liste = Element(1, Box::new(Element(2, Box::new(Element(3, Box::new(Fin))))));
        assert_eq!(somme_liste(&liste), 6);
    }

    #[test]
    fn rc_partage_bien_la_meme_donnee() {
        let config = Rc::new(Configuration {
            nom_application: String::from("test"),
        });
        let clone = Rc::clone(&config);
        assert_eq!(Rc::strong_count(&config), 2);
        assert_eq!(clone.nom_application, "test");
    }

    #[test]
    fn refcell_permet_la_mutation_via_un_acces_immuable() {
        let valeur = RefCell::new(10);
        *valeur.borrow_mut() += 5;
        assert_eq!(*valeur.borrow(), 15);
    }
}
