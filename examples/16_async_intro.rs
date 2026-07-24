// 16 - Introduction a async/await
//
// async/await permet d'ecrire du code asynchrone (qui attend des
// operations d'entree/sortie sans bloquer le thread) avec une syntaxe
// proche du code synchrone. Contrairement aux threads de l'exemple 15,
// une tache asynchrone ne monopolise pas un thread pendant qu'elle
// attend : le runtime peut executer autre chose en attendant.
//
// Le langage Rust fournit async/await, mais pas de runtime : il faut
// une bibliotheque externe pour l'executer. Tokio est la plus utilisee
// (dependance de dev ajoutee dans Cargo.toml pour cet exemple).
//
// Pour lancer cet exemple :
//   cargo run --example 16_async_intro

use std::time::Duration;
use tokio::time::sleep;

// Une fonction async ne s'execute pas quand on l'appelle : elle
// renvoie immediatement un Future (une valeur qui represente "un
// resultat pas encore disponible"). Rien ne se passe tant qu'on ne
// l'attend pas avec .await ou qu'un runtime ne l'execute pas.
async fn recuperer_utilisateur(id: u32) -> String {
    // sleep simule une latence reseau (appel a une base de donnees, une
    // API...). Pendant cette attente, le thread peut executer d'autres
    // taches asynchrones : il n'est pas bloque comme le serait
    // thread::sleep.
    sleep(Duration::from_millis(20)).await;
    format!("utilisateur #{}", id)
}

async fn recuperer_commandes(id_utilisateur: u32) -> Vec<String> {
    sleep(Duration::from_millis(15)).await;
    vec![
        format!("commande A pour {}", id_utilisateur),
        format!("commande B pour {}", id_utilisateur),
    ]
}

// #[tokio::main] transforme cette fonction async en un main() classique
// en demarrant automatiquement un runtime tokio autour d'elle.
#[tokio::main]
async fn main() {
    // Sequentiel : chaque .await attend la fin de l'operation avant de
    // passer a la suivante, comme du code synchrone classique.
    let utilisateur = recuperer_utilisateur(1).await;
    println!("(sequentiel) {}", utilisateur);
    let commandes = recuperer_commandes(1).await;
    println!("(sequentiel) commandes : {:?}", commandes);

    // Concurrent : tokio::join! lance plusieurs futures et les execute
    // en meme temps, sans thread supplementaire. Utile des que deux
    // operations independantes n'ont pas besoin de s'attendre l'une
    // l'autre (ici, deux utilisateurs differents).
    let (utilisateur_2, utilisateur_3) =
        tokio::join!(recuperer_utilisateur(2), recuperer_utilisateur(3));
    println!("(concurrent) {} / {}", utilisateur_2, utilisateur_3);

    // tokio::spawn demarre une tache independante, qui peut continuer
    // en arriere-plan pendant que le reste du code avance. On recupere
    // son resultat en attendant la JoinHandle qu'elle renvoie.
    let tache = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        "resultat de la tache en arriere-plan"
    });

    println!("le main continue pendant que la tache tourne...");
    let resultat_tache = tache.await.expect("la tache ne doit pas paniquer");
    println!("resultat recupere : {}", resultat_tache);

    // Enchainer deux appels dependants : le second a besoin du resultat
    // du premier, donc pas de concurrence possible ici, juste deux
    // .await successifs.
    let profil = recuperer_utilisateur(4).await;
    let commandes_du_profil = recuperer_commandes(4).await;
    println!("profil : {}, commandes : {:?}", profil, commandes_du_profil);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test] est l'equivalent de #[test] pour les fonctions
    // async : il demarre un runtime tokio juste pour ce test.
    #[tokio::test]
    async fn recuperer_utilisateur_renvoie_le_bon_identifiant() {
        let resultat = recuperer_utilisateur(7).await;
        assert_eq!(resultat, "utilisateur #7");
    }

    #[tokio::test]
    async fn recuperer_commandes_renvoie_deux_commandes() {
        let commandes = recuperer_commandes(1).await;
        assert_eq!(commandes.len(), 2);
    }

    #[tokio::test]
    async fn join_execute_bien_les_deux_futures() {
        let (a, b) = tokio::join!(recuperer_utilisateur(1), recuperer_utilisateur(2));
        assert_eq!(a, "utilisateur #1");
        assert_eq!(b, "utilisateur #2");
    }
}
