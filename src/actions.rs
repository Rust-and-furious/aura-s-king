// actions.rs — L'enum central du Command Pattern
// Toutes les actions possibles dans le jeu.

#[derive(Debug)]
pub enum Action {
    Observer,
    Fouiller,
    Ramasser,
    Ouvrir,
    Fermer,
    Utiliser,
    Attaquer { degats: i32 },
    Dialoguer,
    Deplacer { target_zone: usize },
}
