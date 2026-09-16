// 18 - Macros déclaratives
//
// Une macro s'exécute à la compilation, avant que le code ne soit
// vérifié par le compilateur : elle prend du code Rust en entrée
// (sous forme de jetons, pas de valeurs) et produit du code Rust en
// sortie. `macro_rules!` définit ce genre de macro par un ensemble de
// motifs, un peu comme un `match` mais sur la forme du code appelant
// plutôt que sur une valeur.
//
// À la différence d'une fonction, une macro peut accepter un nombre
// variable d'arguments et générer du code différent selon ce qui lui
// est passé : c'est ce que fait `vec![]` dans la bibliothèque standard.
//
// Pour lancer cet exemple :
//   cargo run --example 18_macros

// La forme la plus simple : aucun paramètre, juste un raccourci vers
// un bloc de code fixe.
macro_rules! ligne_de_separation {
    () => {
        println!("----------------------------------------");
    };
}

// `$nom:expr` capture une expression Rust arbitraire et lui donne un
// nom (`nom`) réutilisable à droite du `=>`. Le type de fragment
// (`expr`, `ident`, `ty`...) dit à la macro quelle grammaire accepter :
// ici n'importe quelle expression qui produit une valeur.
macro_rules! double {
    ($valeur:expr) => {
        $valeur * 2
    };
}

// `$(...)`,* répète un motif séparé par des virgules, autant de fois
// qu'il apparaît à l'appel. C'est le mécanisme qui permet à `vec![1,
// 2, 3]` d'accepter un nombre quelconque d'éléments alors qu'une
// fonction a une arité fixe.
macro_rules! somme {
    ($($valeur:expr),*) => {
        {
            // `#[allow(unused_mut)]` : nécessaire uniquement pour l'appel
            // `somme!()` sans argument, où la répétition ci-dessous ne
            // génère aucun `+=` et `mut` devient inutile pour cette
            // expansion précise (clippy analyse chaque site d'appel).
            #[allow(unused_mut)]
            let mut total = 0;
            $(total += $valeur;)*
            total
        }
    };
}

// Une macro peut avoir plusieurs motifs, essayés dans l'ordre comme
// les branches d'un `match`. Ici, la version à un seul argument et la
// version à deux arguments génèrent un message différent.
macro_rules! decrire_personne {
    ($nom:expr) => {
        format!("{} (âge non précisé)", $nom)
    };
    ($nom:expr, $age:expr) => {
        format!("{} ({} ans)", $nom, $age)
    };
}

fn main() {
    ligne_de_separation!();

    let resultat = double!(21);
    println!("double!(21) = {}", resultat);

    ligne_de_separation!();

    let total = somme!(1, 2, 3, 4, 5);
    println!("somme!(1, 2, 3, 4, 5) = {}", total);

    ligne_de_separation!();

    println!("{}", decrire_personne!("Alice"));
    println!("{}", decrire_personne!("Bob", 30));
}

#[cfg(test)]
mod tests {
    // Les macros définies dans ce fichier sont visibles ici sans
    // import supplémentaire : `macro_rules!` place la macro dans la
    // portée du module, pas dans un espace de noms séparé comme les
    // fonctions.

    #[test]
    fn double_multiplie_bien_par_deux() {
        assert_eq!(double!(10), 20);
    }

    #[test]
    fn somme_gere_zero_argument() {
        assert_eq!(somme!(), 0);
    }

    #[test]
    fn somme_additionne_tous_les_arguments() {
        assert_eq!(somme!(1, 2, 3), 6);
    }

    #[test]
    fn decrire_personne_sans_age() {
        assert_eq!(decrire_personne!("Zoé"), "Zoé (âge non précisé)");
    }

    #[test]
    fn decrire_personne_avec_age() {
        assert_eq!(decrire_personne!("Zoé", 8), "Zoé (8 ans)");
    }
}
