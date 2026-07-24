// 15 - Concurrence : threads, Arc<Mutex<T>>, canaux
//
// Rust applique ses regles d'ownership et d'emprunt aussi entre
// threads : la plupart des erreurs classiques de concurrence (data
// race notamment) sont detectees a la compilation plutot qu'a
// l'execution.
//
// Pour lancer cet exemple :
//   cargo run --example 15_concurrency

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // thread::spawn demarre un thread. La closure doit etre `move` des
    // qu'elle utilise une donnee de l'exterieur, pour que le nouveau
    // thread en prenne clairement possession.
    let poignee = thread::spawn(|| {
        for i in 1..=3 {
            println!("thread enfant : etape {}", i);
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..=2 {
        println!("thread principal : etape {}", i);
        thread::sleep(Duration::from_millis(5));
    }

    // join() bloque jusqu'a la fin du thread enfant : sans cet appel,
    // le programme pourrait se terminer avant que le thread ait fini.
    poignee.join().expect("le thread enfant a panique");

    // Arc<T> (Atomic Reference Counted) est l'equivalent thread-safe de
    // Rc<T> : plusieurs threads peuvent partager la possession d'une
    // meme valeur. Mutex<T> garantit qu'un seul thread a la fois peut
    // acceder aux donnees qu'il protege (verrou pose/leve automatiquement
    // via le systeme de portee de Rust).
    let compteur_partage = Arc::new(Mutex::new(0));
    let mut poignees = Vec::new();

    for _ in 0..5 {
        let compteur = Arc::clone(&compteur_partage);
        let poignee = thread::spawn(move || {
            // lock() renvoie un Result (le verrou peut echouer si un
            // autre thread a panique en le detenant) : on utilise
            // unwrap() ici car c'est un cas d'ecole sans thread instable.
            let mut valeur = compteur.lock().unwrap();
            *valeur += 1;
        });
        poignees.push(poignee);
    }

    for poignee in poignees {
        poignee.join().unwrap();
    }

    println!(
        "compteur final apres 5 threads : {}",
        *compteur_partage.lock().unwrap()
    );

    // Canaux (mpsc = multiple producer, single consumer) : une autre
    // maniere de faire communiquer des threads, en s'envoyant des
    // messages plutot qu'en partageant de la memoire.
    let (emetteur, recepteur) = mpsc::channel();

    for id in 0..3 {
        let emetteur_clone = emetteur.clone();
        thread::spawn(move || {
            let message = format!("message du producteur {}", id);
            emetteur_clone.send(message).unwrap();
        });
    }
    drop(emetteur); // sans ce drop, le recepteur attendrait indefiniment

    let mut messages_recus: Vec<String> = recepteur.iter().collect();
    messages_recus.sort();
    for message in &messages_recus {
        println!("recu : {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_mutex_permet_un_compteur_partage_sans_data_race() {
        let compteur = Arc::new(Mutex::new(0));
        let mut poignees = Vec::new();

        for _ in 0..10 {
            let compteur = Arc::clone(&compteur);
            poignees.push(thread::spawn(move || {
                *compteur.lock().unwrap() += 1;
            }));
        }
        for poignee in poignees {
            poignee.join().unwrap();
        }

        assert_eq!(*compteur.lock().unwrap(), 10);
    }

    #[test]
    fn canal_transmet_bien_les_messages() {
        let (emetteur, recepteur) = mpsc::channel();
        emetteur.send(42).unwrap();
        assert_eq!(recepteur.recv().unwrap(), 42);
    }
}
