// 03 - Fonctions
//
// En Rust, chaque parametre de fonction doit etre type, et le type de
// retour est annonce avec `->`. La derniere expression d'un bloc (sans
// point-virgule) en devient la valeur de retour implicite.
//
// Pour lancer cet exemple :
//   cargo run --example 03_functions

// clippy::let_and_return desactive volontairement : dans carre(), on
// garde le `let resultat = { ... }` intermediaire pour bien separer,
// a des fins pedagogiques, "le bloc est une expression" de "la fonction
// retourne sa derniere expression", plutot que de fusionner les deux.
#![allow(clippy::let_and_return)]

fn main() {
    let somme = additionner(3, 4);
    println!("3 + 4 = {}", somme);

    // Une fonction sans valeur de retour renvoie le type unite `()`.
    afficher_ligne_de_titre("Fonctions");

    println!("carre(5) = {}", carre(5));
    println!("est_pair(10) = {}", est_pair(10));

    // Les fonctions sont des valeurs : on peut les passer en parametre.
    println!("resultat via appliquer : {}", appliquer(carre, 6));

    match diviser(10, 0) {
        Some(resultat) => println!("10 / 0 = {}", resultat),
        None => println!("division par zero refusee"),
    }
    match diviser(10, 2) {
        Some(resultat) => println!("10 / 2 = {}", resultat),
        None => println!("division par zero refusee"),
    }
}

// Statement vs expression : dans le corps ci-dessous, `left + right`
// est une expression (pas de point-virgule) donc c'est la valeur
// retournee. On aurait pu ecrire `return left + right;` explicitement.
fn additionner(gauche: i32, droite: i32) -> i32 {
    gauche + droite
}

fn afficher_ligne_de_titre(titre: &str) {
    println!("--- {} ---", titre);
}

// Une fonction peut en appeler une autre et transformer un bloc en
// expression grace aux accolades.
fn carre(n: i32) -> i32 {
    let resultat = {
        let base = n;
        base * base // pas de point-virgule : c'est la valeur du bloc
    };
    resultat
}

fn est_pair(n: i32) -> bool {
    n % 2 == 0
}

// Une fonction peut recevoir une autre fonction en parametre des lors
// que sa signature (ici `fn(i32) -> i32`) correspond.
fn appliquer(f: fn(i32) -> i32, valeur: i32) -> i32 {
    f(valeur)
}

// Retourner un Option<T> est la maniere idiomatique de signaler
// "peut-etre pas de resultat" plutot que de paniquer ou renvoyer -1.
fn diviser(numerateur: i32, denominateur: i32) -> Option<i32> {
    if denominateur == 0 {
        None
    } else {
        Some(numerateur / denominateur)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn additionner_fonctionne() {
        assert_eq!(additionner(2, 3), 5);
    }

    #[test]
    fn diviser_par_zero_renvoie_none() {
        assert_eq!(diviser(10, 0), None);
    }

    #[test]
    fn diviser_renvoie_some_quand_valide() {
        assert_eq!(diviser(10, 2), Some(5));
    }

    #[test]
    fn appliquer_utilise_la_fonction_passee_en_parametre() {
        assert_eq!(appliquer(carre, 4), 16);
    }
}
