// 04 - Structures de controle
//
// if/else, boucles (loop, while, for) et un premier apercu de match.
// Le pattern matching complet (avec enums) arrive dans l'exemple 08.
//
// Pour lancer cet exemple :
//   cargo run --example 04_control_flow

fn main() {
    // if est une expression : on peut l'utiliser directement dans un let.
    let note = 14;
    let mention = if note >= 16 {
        "tres bien"
    } else if note >= 14 {
        "bien"
    } else if note >= 10 {
        "passable"
    } else {
        "insuffisant"
    };
    println!("note {} -> mention {}", note, mention);

    // loop est une boucle infinie qu'on arrete explicitement avec break.
    // break peut aussi renvoyer une valeur, ce qui en fait une expression.
    let mut compteur = 0;
    let resultat = loop {
        compteur += 1;
        if compteur == 5 {
            break compteur * 10;
        }
    };
    println!("loop a boucle jusqu'a {} puis renvoye {}", compteur, resultat);

    // while : condition classique, verifiee avant chaque iteration.
    let mut restant = 3;
    while restant > 0 {
        println!("compte a rebours : {}", restant);
        restant -= 1;
    }

    // for sur un intervalle : 0..5 exclut 5, 0..=5 l'inclut.
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // for sur une collection : la maniere idiomatique de parcourir un
    // tableau ou un vecteur, sans manipuler d'index manuellement.
    let fruits = ["pomme", "banane", "cerise"];
    for fruit in fruits.iter() {
        println!("fruit : {}", fruit);
    }

    // enumerate() donne acces a l'index quand on en a vraiment besoin.
    for (index, fruit) in fruits.iter().enumerate() {
        println!("fruit {} = {}", index, fruit);
    }

    // Les labels de boucle permettent de cibler une boucle externe
    // depuis une boucle imbriquee.
    let mut trouve = None;
    'externe: for x in 0..5 {
        for y in 0..5 {
            if x * y == 6 {
                trouve = Some((x, y));
                break 'externe;
            }
        }
    }
    println!("premier couple (x, y) avec x*y=6 : {:?}", trouve);
}

#[cfg(test)]
mod tests {
    fn mention_pour(note: i32) -> &'static str {
        if note >= 16 {
            "tres bien"
        } else if note >= 14 {
            "bien"
        } else if note >= 10 {
            "passable"
        } else {
            "insuffisant"
        }
    }

    #[test]
    fn mention_correcte_selon_la_note() {
        assert_eq!(mention_pour(18), "tres bien");
        assert_eq!(mention_pour(10), "passable");
        assert_eq!(mention_pour(5), "insuffisant");
    }

    #[test]
    fn loop_peut_renvoyer_une_valeur() {
        let mut compteur = 0;
        let resultat = loop {
            compteur += 1;
            if compteur == 3 {
                break compteur * 100;
            }
        };
        assert_eq!(resultat, 300);
    }
}
