// 12 - Lifetimes (durees de vie)
//
// Les lifetimes n'allongent ni ne raccourcissent la duree de vie d'une
// valeur : ce sont des annotations qui decrivent au compilateur les
// relations entre les durees de vie de plusieurs references, pour
// qu'il puisse garantir qu'aucune ne pointe vers une donnee liberee.
//
// Pour lancer cet exemple :
//   cargo run --example 12_lifetimes

// Sans annotation, le compilateur ne peut pas savoir si la reference
// renvoyee vit aussi longtemps que `a` ou que `b`. 'a ici dit : "la
// valeur de retour vit au moins aussi longtemps que les deux entrees".
fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

// Une struct qui contient une reference doit annoter sa lifetime :
// une instance de Extrait ne peut pas survivre au texte qu'elle
// reference.
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

    let paragraphe = String::from("Rust previent les erreurs memoire. Sans garbage collector.");
    let extrait = Extrait {
        texte: &paragraphe,
    };
    println!("premiere phrase : {}", extrait.premiere_phrase());

    // L'exemple classique qui NE compile PAS (volontairement commente) :
    // la reference renvoyee par plus_long ne peut pas survivre a la
    // variable la plus courte qui a servi a la calculer.
    //
    // let resultat;
    // {
    //     let temporaire = String::from("temporaire");
    //     resultat = plus_long(&s1, &temporaire); // erreur : `temporaire`
    //                                              // ne vit pas assez longtemps
    // }
    // println!("{}", resultat);

    // 'static est une lifetime particuliere : la valeur vit pendant
    // toute la duree du programme. Les litteraux de chaine sont 'static
    // car ils sont stockes directement dans le binaire compile.
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
        let texte = String::from("Premiere phrase. Deuxieme phrase.");
        let extrait = Extrait { texte: &texte };
        assert_eq!(extrait.premiere_phrase(), "Premiere phrase");
    }
}
