// 13 - Closures et itérateurs
//
// Une closure est une fonction anonyme qui peut capturer des variables
// de son environnement. Combinées aux itérateurs, elles permettent
// d'exprimer des transformations de données de façon déclarative,
// sans boucles manuelles.
//
// Pour lancer cet exemple :
//   cargo run --example 13_closures_and_iterators

// clippy::unnecessary_fold désactivé volontairement : fold() est
// utilisé ici pour illustrer la réduction générique, alors que product()
// serait plus court dans ce cas précis. clippy::useless_vec aussi, pour
// la même raison qu'ailleurs dans ce dépôt.
#![allow(clippy::unnecessary_fold, clippy::useless_vec)]

fn main() {
    // Syntaxe de base : paramètres entre |...|, corps ensuite. Le type
    // des paramètres et du retour est le plus souvent inféré.
    let double = |x: i32| x * 2;
    println!("double(5) = {}", double(5));

    // Une closure peut capturer une variable de son environnement par
    // référence (emprunt), contrairement à une fonction classique.
    let facteur = 3;
    let multiplier = |x: i32| x * facteur;
    println!("multiplier(4) = {}", multiplier(4));

    // move force la closure à prendre possession des variables
    // capturées plutôt que de les emprunter : indispensable si la
    // closure doit survivre à la portée actuelle (typiquement pour un
    // thread, voir l'exemple 15).
    let nom = String::from("Rust");
    let saluer = move || println!("Bonjour depuis une closure : {}", nom);
    saluer();

    // --- Itérateurs ---
    let nombres = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map transforme chaque élément, filter garde ceux qui satisfont
    // une condition, collect matérialise le résultat dans une collection.
    let carres_des_pairs: Vec<i32> = nombres
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .collect();
    println!("carrés des nombres pairs : {:?}", carres_des_pairs);

    // sum, fold : réduire une collection à une seule valeur.
    let somme: i32 = nombres.iter().sum();
    println!("somme = {}", somme);

    let produit = nombres.iter().take(4).fold(1, |acc, &n| acc * n);
    println!("produit des 4 premiers = {}", produit);

    // find : renvoie la première valeur qui correspond, enveloppée
    // dans un Option puisqu'aucune ne correspond peut-être.
    let premier_multiple_de_7 = nombres.iter().find(|&&n| n % 7 == 0);
    println!("premier multiple de 7 : {:?}", premier_multiple_de_7);

    // any / all : tests booléens sur l'ensemble de la collection.
    println!("tous positifs ? {}", nombres.iter().all(|&n| n > 0));
    println!("au moins un > 8 ? {}", nombres.iter().any(|&n| n > 8));

    // chain, zip, rev : composer plusieurs itérateurs entre eux.
    let lettres = vec!['a', 'b', 'c'];
    let associations: Vec<(i32, char)> = nombres.iter().cloned().zip(lettres.iter().cloned()).collect();
    println!("zip nombres/lettres : {:?}", associations);

    let ordre_inverse: Vec<&i32> = nombres.iter().rev().take(3).collect();
    println!("3 derniers en ordre inverse : {:?}", ordre_inverse);

    // Une closure comme FnMut : elle modifie une variable capturée.
    let mut total = 0;
    let mut accumuler = |x: i32| total += x;
    accumuler(10);
    accumuler(20);
    println!("total accumulé = {}", total);
}

#[cfg(test)]
mod tests {
    #[test]
    fn filter_puis_map_transforme_correctement() {
        let nombres = vec![1, 2, 3, 4, 5, 6];
        let resultat: Vec<i32> = nombres.iter().filter(|&&n| n % 2 == 0).map(|&n| n * n).collect();
        assert_eq!(resultat, vec![4, 16, 36]);
    }

    #[test]
    fn fold_calcule_un_produit() {
        let nombres = vec![1, 2, 3, 4];
        let produit = nombres.iter().fold(1, |acc, &n| acc * n);
        assert_eq!(produit, 24);
    }

    #[test]
    fn find_renvoie_none_si_rien_ne_correspond() {
        let nombres = vec![1, 2, 3];
        assert_eq!(nombres.iter().find(|&&n| n > 100), None);
    }
}
