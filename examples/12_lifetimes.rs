// 12 - Lifetimes (durées de vie)
//
// Les lifetimes n'allongent ni ne raccourcissent la durée de vie d'une
// valeur : ce sont des annotations qui décrivent au compilateur les
// relations entre les durées de vie de plusieurs références, pour
// qu'il puisse garantir qu'aucune ne pointe vers une donnée libérée.
//
// Pour lancer cet exemple :
//   cargo run --example 12_lifetimes

// Sans annotation, le compilateur ne peut pas savoir si la référence
// renvoyée vit aussi longtemps que `a` ou que `b`. 'a ici dit : "la
// valeur de retour vit au moins aussi longtemps que les deux entrées".
fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

// Une struct qui contient une référence doit annoter sa lifetime :
// une instance de Extrait ne peut pas survivre au texte qu'elle
// référence.
struct Extrait<'a> {
    texte: &'a str,
}

impl<'a> Extrait<'a> {
    fn premiere_phrase(&self) -> &str {
        match self.texte.find('.') {
            Some(index) => &self.texte[..index],
            None => self.texte,
        }
    }
}

fn main() {
    let s1 = String::from("bonjour");
    let s2 = String::from("salutations amicales");
    println!("le plus long : {}", plus_long(&s1, &s2));

    let paragraphe = String::from("Rust prévient les erreurs mémoire. Sans garbage collector.");
    let extrait = Extrait {
        texte: &paragraphe,
    };
    println!("première phrase : {}", extrait.premiere_phrase());

    // L'exemple classique qui NE compile PAS (volontairement commenté) :
    // la référence renvoyée par plus_long ne peut pas survivre à la
    // variable la plus courte qui a servi à la calculer.
    //
    // let resultat;
    // {
    //     let temporaire = String::from("temporaire");
    //     resultat = plus_long(&s1, &temporaire); // erreur : `temporaire`
    //                                              // ne vit pas assez longtemps
    // }
    // println!("{}", resultat);

    // 'static est une lifetime particulière : la valeur vit pendant
    // toute la durée du programme. Les littéraux de chaîne sont 'static
    // car ils sont stockés directement dans le binaire compilé.
    let toujours_valide: &'static str = "je vis aussi longtemps que le programme";
    println!("{}", toujours_valide);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_long_renvoie_la_bonne_chaine() {
        assert_eq!(plus_long("abc", "abcdef"), "abcdef");
        assert_eq!(plus_long("abcdef", "xy"), "abcdef");
    }

    #[test]
    fn premiere_phrase_s_arrete_au_premier_point() {
        let texte = String::from("Première phrase. Deuxième phrase.");
        let extrait = Extrait { texte: &texte };
        assert_eq!(extrait.premiere_phrase(), "Première phrase");
    }
}
