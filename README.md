# learning-rust

Parcours d'apprentissage du langage Rust, du zero absolu jusqu'aux bases
de la concurrence et de l'asynchrone. Seize exemples independants,
commentes en francais, chacun executable et teste isolement.

Version anglaise : [README.en.md](README.en.md)

## Pourquoi ce depot

Pas un tutoriel a lire, un ensemble de programmes courts a executer,
modifier et casser volontairement pour voir ce que le compilateur
Rust en dit. Chaque fichier de `examples/` couvre une notion, dans
l'ordre ou il est logique de les decouvrir.

## Prerequis

- [Rust](https://www.rust-lang.org/) installe via [rustup](https://rustup.rs/) :

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Aucune autre dependance : `cargo` (fourni avec Rust) suffit pour tout
  compiler, executer et tester.

## Lancer un exemple

Chaque fichier de `examples/` est un mini-programme autonome.

```bash
cargo run --example 01_hello_world
cargo run --example 08_enums_and_pattern_matching
```

## Lancer les tests

Chaque exemple contient ses propres tests unitaires (`#[cfg(test)]`).
Une seule commande execute tout le depot :

```bash
cargo test
```

## Verifier le style (optionnel)

Le depot est propre sous [clippy](https://github.com/rust-lang/rust-clippy),
le linter officiel de Rust :

```bash
rustup component add clippy
cargo clippy --examples --all-targets
```

Quelques avertissements clippy sont volontairement desactives (via
`#![allow(...)]` en tete de fichier, avec le commentaire qui explique
pourquoi) quand la forme "idiomatique" masquerait la notion en cours
d'explication.

## Sommaire des exemples

| # | Fichier | Notion |
|---|---------|--------|
| 01 | [`hello_world`](examples/01_hello_world.rs) | Premier programme, `println!`, commentaires |
| 02 | [`variables_and_types`](examples/02_variables_and_types.rs) | Variables, mutabilite, shadowing, types scalaires, tuples, tableaux |
| 03 | [`functions`](examples/03_functions.rs) | Fonctions, expressions vs instructions, `Option` en retour |
| 04 | [`control_flow`](examples/04_control_flow.rs) | `if`/`else`, `loop`, `while`, `for`, labels de boucle |
| 05 | [`ownership`](examples/05_ownership.rs) | Possession, move, `Copy`, `clone`, portee et `drop` |
| 06 | [`borrowing_and_references`](examples/06_borrowing_and_references.rs) | Emprunts `&`/`&mut`, regles du borrow checker, slices |
| 07 | [`structs`](examples/07_structs.rs) | Structs, `impl`, methodes, fonctions associees |
| 08 | [`enums_and_pattern_matching`](examples/08_enums_and_pattern_matching.rs) | Enums, `match` exhaustif, `Option`, `if let` |
| 09 | [`collections`](examples/09_collections.rs) | `Vec`, `String`, `HashMap` |
| 10 | [`error_handling`](examples/10_error_handling.rs) | `Option`, `Result`, l'operateur `?`, `panic!` |
| 11 | [`generics_and_traits`](examples/11_generics_and_traits.rs) | Generiques, traits, methodes par defaut, `dyn Trait` |
| 12 | [`lifetimes`](examples/12_lifetimes.rs) | Annotations de duree de vie, structs avec references, `'static` |
| 13 | [`closures_and_iterators`](examples/13_closures_and_iterators.rs) | Closures, capture d'environnement, adaptateurs d'iterateurs |
| 14 | [`smart_pointers`](examples/14_smart_pointers.rs) | `Box`, `Rc`, `RefCell`, mutabilite interieure |
| 15 | [`concurrency`](examples/15_concurrency.rs) | Threads, `Arc<Mutex<T>>`, canaux `mpsc` |
| 16 | [`async_intro`](examples/16_async_intro.rs) | `async`/`await`, runtime Tokio, `join!`, `spawn` |

## Structure du depot

Un seul crate Cargo (`learning-rust`), sans code de bibliotheque
(`src/lib.rs` est volontairement vide de logique). Tout le contenu
pedagogique vit dans `examples/`, ou chaque fichier est a la fois un
binaire executable et un module de tests.

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

## Etat d'avancement

Les 16 notions listees ci-dessus sont ecrites, testees et verifiees
avec clippy. Pistes pour la suite, non prevues a une date precise :
modules et organisation de crate multi-fichiers, macros declaratives,
traits `Iterator`/`From`/`Into` ecrits a la main, un petit projet de
synthese (CLI) qui recombine plusieurs notions.

## Licence

[MIT](LICENSE) - (c) 2026 Riadh MNASRI
