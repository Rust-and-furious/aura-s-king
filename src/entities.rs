// entities.rs — Le trait Interactable et toutes les entités concrètes du jeu.
// Les entités illustrées ici (Garde, Fenetre, Pomme) sont des exemples.

use crate::actions::Action;
use crate::player::Player;
use crate::traits::{Fightable, Openable, Useable};
use crate::world::WorldManager;

// ============================================================
// Le Trait principal — Interface avec le moteur de jeu
// ============================================================

pub trait Interactable {
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    /// L'entité déclare dynamiquement ce que le joueur peut faire avec elle.
    fn get_actions(&self, player: &Player, world: &WorldManager) -> Vec<Action>;

    /// Le moteur de jeu déclenche l'action choisie par le joueur.
    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager);
}

// ============================================================
// Exemple 1 : Garde (implémente Fightable)
// ============================================================

pub struct Garde {
    pub name: String,
    pub description: String,
    pub hp: i32,
    pub is_hostile: bool,
}

impl Fightable for Garde {
    fn recevoir_degats(&mut self, degats: i32) {
        self.hp -= degats;
        println!("{} reçoit {} dégâts ! (HP restants : {})", self.name, degats, self.hp);
    }

    fn est_vivant(&self) -> bool {
        self.hp > 0
    }
}

impl Interactable for Garde {
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { &self.description }

    fn get_actions(&self, _player: &Player, _world: &WorldManager) -> Vec<Action> {
        let mut actions = vec![Action::Observer, Action::Dialoguer];
        if self.is_hostile {
            actions.push(Action::Attaquer { degats: 10 });
        }
        actions
    }

    fn execute_action(&mut self, action: &Action, _player: &mut Player, _world: &mut WorldManager) {
        match action {
            Action::Observer => println!("Vous observez {}. {}", self.name, self.description),
            Action::Dialoguer => println!("{} vous toise en silence.", self.name),
            Action::Attaquer { degats } => {
                // Délégation au trait de capacité Fightable
                self.recevoir_degats(*degats);
                if !self.est_vivant() {
                    println!("{} est vaincu !", self.name);
                }
            }
            _ => println!("Impossible de faire ça ici."),
        }
    }
}

// ============================================================
// Exemple 2 : Fenêtre (implémente Openable)
// ============================================================

pub struct Fenetre {
    pub name: String,
    pub description: String,
    pub est_ouverte: bool,
    pub est_cassee: bool,
}

impl Openable for Fenetre {
    fn ouvrir(&mut self) -> Result<(), &'static str> {
        if self.est_cassee { return Err("Impossible, la fenêtre est cassée !"); }
        if self.est_ouverte { return Err("C'est déjà ouvert."); }
        self.est_ouverte = true;
        Ok(())
    }

    fn fermer(&mut self) -> Result<(), &'static str> {
        if !self.est_ouverte { return Err("C'est déjà fermé."); }
        self.est_ouverte = false;
        Ok(())
    }
}

impl Interactable for Fenetre {
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { &self.description }

    fn get_actions(&self, _player: &Player, _world: &WorldManager) -> Vec<Action> {
        let mut actions = vec![Action::Observer];
        if !self.est_cassee {
            if self.est_ouverte { actions.push(Action::Fermer); }
            else { actions.push(Action::Ouvrir); }
        }
        actions
    }

    fn execute_action(&mut self, action: &Action, _player: &mut Player, _world: &mut WorldManager) {
        match action {
            Action::Observer => println!("Vous regardez la fenêtre. {}", self.description),
            Action::Ouvrir => match self.ouvrir() {
                Ok(_) => println!("Vous avez ouvert la fenêtre."),
                Err(e) => println!("{}", e),
            },
            Action::Fermer => match self.fermer() {
                Ok(_) => println!("Vous avez fermé la fenêtre."),
                Err(e) => println!("{}", e),
            },
            _ => println!("Impossible de faire ça ici."),
        }
    }
}

// ============================================================
// Exemple 3 : Pomme (implémente Useable)
// ============================================================

pub struct Pomme {
    pub name: String,
    pub description: String,
    pub aura_rendue: f64,
}

impl Useable for Pomme {
    fn utiliser(&mut self, player: &mut Player, _world: &mut WorldManager) -> Result<(), &'static str> {
        println!("Vous mangez {}. Vous récupérez {} aura !", self.name, self.aura_rendue);
        player.aura += self.aura_rendue;
        Ok(())
    }
}

impl Interactable for Pomme {
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { &self.description }

    fn get_actions(&self, _player: &Player, _world: &WorldManager) -> Vec<Action> {
        vec![Action::Observer, Action::Ramasser, Action::Utiliser]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => println!("Vous regardez {}. {}", self.name, self.description),
            Action::Ramasser => println!("Vous ramassez {}.", self.name),
            Action::Utiliser => match self.utiliser(player, world) {
                Ok(_) => println!("(Aura actuelle : {})", player.aura),
                Err(e) => println!("{}", e),
            },
            _ => println!("Impossible de faire ça ici."),
        }
    }
}
