# learning-rust

Parcours d'apprentissage du langage Rust, du zéro absolu jusqu'aux bases
de la concurrence et de l'asynchrone. Seize exemples indépendants,
commentés en français, chacun exécutable et testé isolément.

Version anglaise : [README.en.md](README.en.md)

## Pourquoi ce dépôt

Pas un tutoriel à lire, un ensemble de programmes courts à exécuter,
modifier et casser volontairement pour voir ce que le compilateur
Rust en dit. Chaque fichier de `examples/` couvre une notion, dans
l'ordre où il est logique de les découvrir.

## Prérequis

- [Rust](https://www.rust-lang.org/) installé via [rustup](https://rustup.rs/) :

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Aucune autre dépendance : `cargo` (fourni avec Rust) suffit pour tout
  compiler, exécuter et tester.

## Lancer un exemple

Chaque fichier de `examples/` est un mini-programme autonome.

```bash
cargo run --example 01_hello_world
cargo run --example 08_enums_and_pattern_matching
```

## Lancer les tests

Chaque exemple contient ses propres tests unitaires (`#[cfg(test)]`).
Une seule commande exécute tout le dépôt :

```bash
cargo test
```

## Vérifier le style (optionnel)

Le dépôt est propre sous [clippy](https://github.com/rust-lang/rust-clippy),
le linter officiel de Rust :

```bash
rustup component add clippy
cargo clippy --examples --all-targets
```

Quelques avertissements clippy sont volontairement désactivés (via
`#![allow(...)]` en tête de fichier, avec le commentaire qui explique
pourquoi) quand la forme "idiomatique" masquerait la notion en cours
d'explication.

## Sommaire des exemples

| # | Fichier | Notion |
|---|---------|--------|
| 01 | [`hello_world`](examples/01_hello_world.rs) | Premier programme, `println!`, commentaires |
| 02 | [`variables_and_types`](examples/02_variables_and_types.rs) | Variables, mutabilité, shadowing, types scalaires, tuples, tableaux |
| 03 | [`functions`](examples/03_functions.rs) | Fonctions, expressions vs instructions, `Option` en retour |
| 04 | [`control_flow`](examples/04_control_flow.rs) | `if`/`else`, `loop`, `while`, `for`, labels de boucle |
| 05 | [`ownership`](examples/05_ownership.rs) | Possession, move, `Copy`, `clone`, portée et `drop` |
| 06 | [`borrowing_and_references`](examples/06_borrowing_and_references.rs) | Emprunts `&`/`&mut`, règles du borrow checker, slices |
| 07 | [`structs`](examples/07_structs.rs) | Structs, `impl`, méthodes, fonctions associées |
| 08 | [`enums_and_pattern_matching`](examples/08_enums_and_pattern_matching.rs) | Enums, `match` exhaustif, `Option`, `if let` |
| 09 | [`collections`](examples/09_collections.rs) | `Vec`, `String`, `HashMap` |
| 10 | [`error_handling`](examples/10_error_handling.rs) | `Option`, `Result`, l'opérateur `?`, `panic!` |
| 11 | [`generics_and_traits`](examples/11_generics_and_traits.rs) | Génériques, traits, méthodes par défaut, `dyn Trait` |
| 12 | [`lifetimes`](examples/12_lifetimes.rs) | Annotations de durée de vie, structs avec références, `'static` |
| 13 | [`closures_and_iterators`](examples/13_closures_and_iterators.rs) | Closures, capture d'environnement, adaptateurs d'itérateurs |
| 14 | [`smart_pointers`](examples/14_smart_pointers.rs) | `Box`, `Rc`, `RefCell`, mutabilité intérieure |
| 15 | [`concurrency`](examples/15_concurrency.rs) | Threads, `Arc<Mutex<T>>`, canaux `mpsc` |
| 16 | [`async_intro`](examples/16_async_intro.rs) | `async`/`await`, runtime Tokio, `join!`, `spawn` |

## Structure du dépôt

Un seul crate Cargo (`learning-rust`), sans code de bibliothèque
(`src/lib.rs` est volontairement vide de logique). Tout le contenu
pédagogique vit dans `examples/`, où chaque fichier est à la fois un
binaire exécutable et un module de tests.

```
learning-rust/
  Cargo.toml
  examples/
    01_hello_world.rs
    ...
    16_async_intro.rs
  src/
    lib.rs
```

## État d'avancement

Les 16 notions listées ci-dessus sont écrites, testées et vérifiées
avec clippy. Pistes pour la suite, non prévues à une date précise :
modules et organisation de crate multi-fichiers, macros déclaratives,
traits `Iterator`/`From`/`Into` écrits à la main, un petit projet de
synthèse (CLI) qui recombine plusieurs notions.

## Licence

[MIT](LICENSE) - (c) 2026 Riadh MNASRI
