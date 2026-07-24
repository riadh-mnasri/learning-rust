// 02 - Variables et types
//
// Rust est statiquement typé : chaque valeur a un type connu à la
// compilation. Le compilateur infère souvent ce type, mais on peut
// toujours l'annoter explicitement.
//
// Pour lancer cet exemple :
//   cargo run --example 02_variables_and_types

fn main() {
    // Par défaut, une variable est immuable. Cette ligne ne compilerait
    // pas si on essayait de réaffecter `age` plus loin sans `mut`.
    let age: u32 = 33;
    println!("age = {}", age);

    // `mut` rend la variable modifiable.
    let mut compteur = 0;
    compteur += 1;
    compteur += 1;
    println!("compteur = {}", compteur);

    // Shadowing : on peut redéclarer une variable du même nom, y compris
    // avec un type différent. Ce n'est pas de la mutation, c'est une
    // nouvelle variable qui masque l'ancienne.
    let valeur = "42";
    let valeur: i32 = valeur.parse().expect("devrait être un nombre");
    let valeur = valeur * 2;
    println!("valeur après shadowing = {}", valeur);

    // Types scalaires principaux : entiers (signés/non signés, 8 à 128
    // bits), flottants, booléens, caractères.
    let entier_signe: i32 = -7;
    let entier_non_signe: u8 = 255;
    let flottant: f64 = 9.81;
    let vrai_ou_faux: bool = true;
    let caractere: char = 'R'; // un char Rust est un scalaire Unicode (4 octets)

    println!(
        "i32={} u8={} f64={} bool={} char={}",
        entier_signe, entier_non_signe, flottant, vrai_ou_faux, caractere
    );

    // Tuple : regroupe des valeurs de types différents, taille fixe.
    let personne: (&str, u32, bool) = ("Riadh", 33, true);
    let (nom, age_personne, actif) = personne; // destructuration
    println!("{} a {} ans, actif = {}", nom, age_personne, actif);
    println!("accès par index : {}", personne.0);

    // Tableau : taille fixe, éléments du même type, alloué sur la pile.
    let semaine: [&str; 7] = ["lun", "mar", "mer", "jeu", "ven", "sam", "dim"];
    println!("jour 3 = {}", semaine[2]);
    println!("nombre de jours = {}", semaine.len());

    // Une constante doit avoir un type explicite et une valeur connue
    // à la compilation. Convention : SCREAMING_SNAKE_CASE.
    const LIMITE_TENTATIVES: u32 = 3;
    println!("limite de tentatives = {}", LIMITE_TENTATIVES);
}

#[cfg(test)]
mod tests {
    #[test]
    fn le_shadowing_change_bien_le_type_et_la_valeur() {
        let valeur = "42";
        let valeur: i32 = valeur.parse().unwrap();
        let valeur = valeur * 2;
        assert_eq!(valeur, 84);
    }

    #[test]
    fn destructuration_de_tuple() {
        let personne = ("Alice", 30);
        let (nom, age) = personne;
        assert_eq!(nom, "Alice");
        assert_eq!(age, 30);
    }
}
