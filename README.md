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

Le projet respecte une séparation stricte entre le **moteur** (code Rust) et les **données** (`data/world.json`). Modules :
- `src/main.rs` : point d'entrée + boucle de jeu (navigation entre zones, points d'intérêt, déplacement).
- `src/actions.rs` : `enum Action` (Command Pattern — tous les verbes).
- `src/traits.rs` : traits de capacité `Openable`, `Fightable`, `Useable`.
- `src/loader.rs` : chargement du monde depuis `data/world.json` (DTOs serde + factory `build_entity`).
- `src/world.rs` : `WorldManager`, `Zone`, `InterestPoint`.
- `src/player.rs` : le joueur et ses attributs.
- `src/menu.rs` : menus clavier interactifs (crossterm).
- `src/couleur.rs` : macro `colore!` (couleurs ANSI).
- `src/audio.rs` : lecture des sons.
- `src/save.rs` : sauvegarde / chargement de l'état en JSON (trait `Saveable`).
- `src/entities/` : une entité concrète par fichier ; `mod.rs` définit les traits `Interactable` et `Saveable`.

### Contenu du jeu

Zones jouables : **Maison, Plaine, Forêt, Cimetière, Lac, Île, Village, Château, Salle du trône** — **le jeu est complet**. Le but : impressionner le Roi Anthony (atteindre **1 000 000 d'aura** avant 20h00) pour être adoubé chevalier. On se déplace entre zones reliées via l'option **« Se déplacer »**, ou par des entités-passages (porte, fenêtre, barque). Le scénario complet est décrit dans `histoire.md`.
