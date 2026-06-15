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


### Activer le son (Linux)

Le jeu utilise la bibliothèque **`rodio`** pour la musique et les bruitages, qui s'appuie sur le système audio **ALSA / PulseAudio**. Sous Linux, il faut installer les paquets correspondants avant de lancer le jeu :

```bash
sudo apt update
sudo apt install pulseaudio-utils alsa-utils
```

> **Note :** si la compilation échoue avec une erreur liée à ALSA, installez également la bibliothèque de développement nécessaire à `rodio` :
>
> ```bash
> sudo apt install libasound2-dev
> ```
