// 19 - Implémenter soi-même des traits standard : Iterator, From, Into
//
// L'exemple 13 utilisait déjà des itérateurs (`.map`, `.filter`,
// `.sum`...) sur des types fournis par la bibliothèque standard. Ici,
// on écrit ces traits nous-mêmes pour un type qui nous appartient : la
// mécanique derrière `for` et les conversions `.into()` devient
// visible plutôt que magique.
//
// Pour lancer cet exemple :
//   cargo run --example 19_custom_traits

// --- Iterator ---
//
// Implémenter `Iterator` demande une seule méthode, `next`, qui
// renvoie `Some(valeur)` tant qu'il reste quelque chose à produire, et
// `None` une fois terminé. En échange, le type obtient gratuitement
// tous les adaptateurs de l'exemple 13 (`.map`, `.take`, `.sum`...) et
// devient utilisable dans une boucle `for`.
struct Fibonacci {
    actuel: u64,
    suivant: u64,
}

impl Fibonacci {
    fn nouveau() -> Self {
        Fibonacci {
            actuel: 0,
            suivant: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let valeur = self.actuel;
        self.actuel = self.suivant;
        self.suivant += valeur;
        // Toujours Some : cette suite est infinie. C'est au code
        // appelant de la limiter, par exemple avec `.take(n)`.
        Some(valeur)
    }
}

// --- From / Into ---
//
// Implémenter `From<A> for B` définit comment obtenir un B à partir
// d'un A. Rust fournit alors automatiquement `Into<B> for A` en
// retour (les deux traits sont liés) : les deux syntaxes ci-dessous
// appellent la même conversion.
struct Minutes(u32);
struct Secondes(u32);

impl From<Minutes> for Secondes {
    fn from(minutes: Minutes) -> Self {
        Secondes(minutes.0 * 60)
    }
}

fn afficher_en_secondes(duree: Secondes) {
    println!("{} secondes", duree.0);
}

fn main() {
    println!("les 8 premiers termes de Fibonacci :");
    let termes: Vec<u64> = Fibonacci::nouveau().take(8).collect();
    println!("{:?}", termes);

    // Puisque Fibonacci implémente Iterator, il fonctionne directement
    // dans un `for`, exactement comme un Vec ou un Range.
    println!("somme des termes pairs sous 50 :");
    let somme: u64 = Fibonacci::nouveau().take_while(|&n| n < 50).filter(|n| n % 2 == 0).sum();
    println!("{}", somme);

    // `From` explicite : Secondes::from(Minutes(2)).
    let duree_a = Secondes::from(Minutes(2));
    afficher_en_secondes(duree_a);

    // `.into()` : la même conversion, mais son type de départ (Minutes)
    // se déduit de ce que la fonction appelée attend en argument.
    let duree_b: Secondes = Minutes(5).into();
    afficher_en_secondes(duree_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_produit_la_suite_attendue() {
        let termes: Vec<u64> = Fibonacci::nouveau().take(6).collect();
        assert_eq!(termes, vec![0, 1, 1, 2, 3, 5]);
    }

    #[test]
    fn fibonacci_se_combine_avec_les_adaptateurs_standard() {
        let somme: u64 = Fibonacci::nouveau().take(5).sum();
        // Somme volontairement écrite terme à terme (plutôt que `7`) pour
        // rester lisible à côté des 5 premiers termes de la suite.
        #[allow(clippy::identity_op)]
        let attendu = 0 + 1 + 1 + 2 + 3;
        assert_eq!(somme, attendu);
    }

    #[test]
    fn from_convertit_minutes_en_secondes() {
        let secondes = Secondes::from(Minutes(3));
        assert_eq!(secondes.0, 180);
    }

    #[test]
    fn into_utilise_la_meme_conversion_que_from() {
        let secondes: Secondes = Minutes(1).into();
        assert_eq!(secondes.0, 60);
    }
}
