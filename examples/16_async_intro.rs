// 16 - Introduction à async/await
//
// async/await permet d'écrire du code asynchrone (qui attend des
// opérations d'entrée/sortie sans bloquer le thread) avec une syntaxe
// proche du code synchrone. Contrairement aux threads de l'exemple 15,
// une tâche asynchrone ne monopolise pas un thread pendant qu'elle
// attend : le runtime peut exécuter autre chose en attendant.
//
// Le langage Rust fournit async/await, mais pas de runtime : il faut
// une bibliothèque externe pour l'exécuter. Tokio est la plus utilisée
// (dépendance de dev ajoutée dans Cargo.toml pour cet exemple).
//
// Pour lancer cet exemple :
//   cargo run --example 16_async_intro

use std::time::Duration;
use tokio::time::sleep;

// Une fonction async ne s'exécute pas quand on l'appelle : elle
// renvoie immédiatement un Future (une valeur qui représente "un
// résultat pas encore disponible"). Rien ne se passe tant qu'on ne
// l'attend pas avec .await ou qu'un runtime ne l'exécute pas.
async fn recuperer_utilisateur(id: u32) -> String {
    // sleep simule une latence réseau (appel à une base de données, une
    // API...). Pendant cette attente, le thread peut exécuter d'autres
    // tâches asynchrones : il n'est pas bloqué comme le serait
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
// en démarrant automatiquement un runtime tokio autour d'elle.
#[tokio::main]
async fn main() {
    // Séquentiel : chaque .await attend la fin de l'opération avant de
    // passer à la suivante, comme du code synchrone classique.
    let utilisateur = recuperer_utilisateur(1).await;
    println!("(séquentiel) {}", utilisateur);
    let commandes = recuperer_commandes(1).await;
    println!("(séquentiel) commandes : {:?}", commandes);

    // Concurrent : tokio::join! lance plusieurs futures et les exécute
    // en même temps, sans thread supplémentaire. Utile dès que deux
    // opérations indépendantes n'ont pas besoin de s'attendre l'une
    // l'autre (ici, deux utilisateurs différents).
    let (utilisateur_2, utilisateur_3) =
        tokio::join!(recuperer_utilisateur(2), recuperer_utilisateur(3));
    println!("(concurrent) {} / {}", utilisateur_2, utilisateur_3);

    // tokio::spawn démarre une tâche indépendante, qui peut continuer
    // en arrière-plan pendant que le reste du code avance. On récupère
    // son résultat en attendant la JoinHandle qu'elle renvoie.
    let tache = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        "résultat de la tâche en arrière-plan"
    });

    println!("le main continue pendant que la tâche tourne...");
    let resultat_tache = tache.await.expect("la tâche ne doit pas paniquer");
    println!("résultat récupéré : {}", resultat_tache);

    // Enchaîner deux appels dépendants : le second a besoin du résultat
    // du premier, donc pas de concurrence possible ici, juste deux
    // .await successifs.
    let profil = recuperer_utilisateur(4).await;
    let commandes_du_profil = recuperer_commandes(4).await;
    println!("profil : {}, commandes : {:?}", profil, commandes_du_profil);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test] est l'équivalent de #[test] pour les fonctions
    // async : il démarre un runtime tokio juste pour ce test.
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
