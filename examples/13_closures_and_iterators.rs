// 13 - Closures et iterateurs
//
// Une closure est une fonction anonyme qui peut capturer des variables
// de son environnement. Combinees aux iterateurs, elles permettent
// d'exprimer des transformations de donnees de facon declarative,
// sans boucles manuelles.
//
// Pour lancer cet exemple :
//   cargo run --example 13_closures_and_iterators

// clippy::unnecessary_fold desactive volontairement : fold() est
// utilise ici pour illustrer la reduction generique, alors que product()
// serait plus court dans ce cas precis. clippy::useless_vec aussi, pour
// la meme raison qu'ailleurs dans ce depot.
#![allow(clippy::unnecessary_fold, clippy::useless_vec)]

fn main() {
    // Syntaxe de base : parametres entre |...|, corps ensuite. Le type
    // des parametres et du retour est le plus souvent infere.
    let double = |x: i32| x * 2;
    println!("double(5) = {}", double(5));

    // Une closure peut capturer une variable de son environnement par
    // reference (emprunt), contrairement a une fonction classique.
    let facteur = 3;
    let multiplier = |x: i32| x * facteur;
    println!("multiplier(4) = {}", multiplier(4));

    // move force la closure a prendre possession des variables
    // capturees plutot que de les emprunter : indispensable si la
    // closure doit survivre a la portee actuelle (typiquement pour un
    // thread, voir l'exemple 15).
    let nom = String::from("Rust");
    let saluer = move || println!("Bonjour depuis une closure : {}", nom);
    saluer();

    // --- Iterateurs ---
    let nombres = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map transforme chaque element, filter garde ceux qui satisfont
    // une condition, collect materialise le resultat dans une collection.
    let carres_des_pairs: Vec<i32> = nombres
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .collect();
    println!("carres des nombres pairs : {:?}", carres_des_pairs);

    // sum, fold : reduire une collection a une seule valeur.
    let somme: i32 = nombres.iter().sum();
    println!("somme = {}", somme);

    let produit = nombres.iter().take(4).fold(1, |acc, &n| acc * n);
    println!("produit des 4 premiers = {}", produit);

    // find : renvoie la premiere valeur qui correspond, enveloppee
    // dans un Option puisqu'aucune ne correspond peut-etre.
    let premier_multiple_de_7 = nombres.iter().find(|&&n| n % 7 == 0);
    println!("premier multiple de 7 : {:?}", premier_multiple_de_7);

    // any / all : tests booleens sur l'ensemble de la collection.
    println!("tous positifs ? {}", nombres.iter().all(|&n| n > 0));
    println!("au moins un > 8 ? {}", nombres.iter().any(|&n| n > 8));

    // chain, zip, rev : composer plusieurs iterateurs entre eux.
    let lettres = vec!['a', 'b', 'c'];
    let associations: Vec<(i32, char)> = nombres.iter().cloned().zip(lettres.iter().cloned()).collect();
    println!("zip nombres/lettres : {:?}", associations);

    let ordre_inverse: Vec<&i32> = nombres.iter().rev().take(3).collect();
    println!("3 derniers en ordre inverse : {:?}", ordre_inverse);

    // Une closure comme FnMut : elle modifie une variable capturee.
    let mut total = 0;
    let mut accumuler = |x: i32| total += x;
    accumuler(10);
    accumuler(20);
    println!("total accumule = {}", total);
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
