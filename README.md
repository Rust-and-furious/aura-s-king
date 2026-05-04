# Aura's King

## Comment lancer le jeu

Assurez-vous d'avoir installé [Rust et Cargo](https://www.rust-lang.org/tools/install) sur votre machine.

### Exécuter le projet


```bash
cargo run
```

### Vérifier le projet (sans l'exécuter)

```bash
cargo check
```

### Architecture du code

Le projet est divisé en plusieurs modules logiques :
- `src/main.rs` : Le point d'entrée de l'application.
- `src/world.rs` : Contient tout ce qui a trait au monde (`WorldManager`, `Zone`, `InterestPoint`).
- `src/player.rs` : Représente le joueur et ses attributs.
- `src/entities.rs` : Définit le trait `Interactable` ainsi que les entités physiques avec lesquelles on peut interagir (`Npc`, `Furniture`, `Objet`).
- `src/actions.rs` : Regroupe les différentes actions possibles en interagissant avec l'environnement (`Dialogue`, `Fouiller`, `Ramasser`).
