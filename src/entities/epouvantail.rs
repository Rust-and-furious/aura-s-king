use crate::actions::Action;
use crate::entities::Interactable;
use crate::player::Player;
use crate::traits::Fightable;
use crate::world::WorldManager;

pub struct Epouvantail {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub chapeau_id: usize,
    pub chapeau_pris: bool,
    pub deja_attaque: bool,
}

impl Fightable for Epouvantail {
    // un tas de paille n'a pas de points de vie : on peut le frapper, pas le "tuer"
    fn recevoir_degats(&mut self, _degats: i32) {}

    fn est_vivant(&self) -> bool {
        false
    }
}

impl Interactable for Epouvantail {
    fn id(&self) -> usize {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }

    fn get_actions(&self, _player: &Player, _world: &WorldManager) -> Vec<Action> {
        vec![
            Action::Observer,
            Action::Ramasser,
            Action::Attaquer { degats: 5 },
            Action::Dialoguer,
        ]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Un épouvantail. Il a l'air plus chevaleresque que vous. C'est vexant.");
            }
            Action::Ramasser => {
                world.current_tick += 3;
                if self.chapeau_pris {
                    println!("Vous avez déjà volé son chapeau. Il a l'air encore plus pitoyable sans.");
                } else {
                    self.chapeau_pris = true;
                    player.inventory.push(self.chapeau_id);
                    player.aura += 15000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("\x1B[32m[+15 000 Aura]\x1B[0m Vous volez le chapeau de l'épouvantail. Petit larcin, grande classe.");
                }
            }
            Action::Attaquer { degats } => {
                self.recevoir_degats(*degats);
                world.current_tick += 10;
                // les corbeaux ne sont impressionnés qu'à la première victoire (pas une mine d'aura)
                if self.deja_attaque {
                    println!("Vous re-frappez le bâton habillé. Les corbeaux, blasés, ne décollent même pas.");
                } else {
                    self.deja_attaque = true;
                    player.aura += 25000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("\x1B[32m[+25 000 Aura]\x1B[0m Les corbeaux sont impressionnés : vous avez gagné un duel contre un bâton habillé.");
                }
            }
            Action::Dialoguer => {
                world.current_tick += 5;
                player.aura -= 50000.0;
                crate::audio::play_sound("assets/defeat.wav");
                println!("\x1B[31m[-50 000 Aura]\x1B[0m Il ne répond pas. Évidemment. Vous venez de parler à un tas de paille devant un corbeau moqueur.");
            }
            _ => println!("Action impossible sur l'épouvantail."),
        }
    }
}
