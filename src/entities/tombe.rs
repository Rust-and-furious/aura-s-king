use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Tombe {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub ver_id: usize,
    pub ver_trouve: bool,
}

impl Saveable for Tombe {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("ver_trouve".to_string(), serde_json::json!(self.ver_trouve));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("ver_trouve") {
            if let Some(b) = val.as_bool() {
                self.ver_trouve = b;
            }
        }
    }
}

impl Useable for Tombe {
    // s'allonger dans la tombe : une perte de temps déprimante, toujours
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        world.current_tick += 60;
        player.aura -= 30000.0;
        crate::audio::play_sound("assets/defeat.wav");
        println!("{} Vous testez le confort. C'est ferme. Vous perdez un temps précieux à déprimer au fond d'un trou.", colore!(Rouge, "[-30 000 Aura]"));
        Ok(())
    }
}

impl Interactable for Tombe {
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
        vec![Action::Observer, Action::Utiliser, Action::Fouiller]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Il n'y a pas de nom. Mais les dimensions correspondent curieusement à votre taille et à votre carrure. C'est sûrement une coïncidence.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            Action::Fouiller => {
                world.current_tick += 15;
                if jet_reussite(world.current_tick, 50) {
                    // un seul ver à dénicher : pas une rente d'aura
                    if self.ver_trouve {
                        println!("Vous ne remontez qu'une poignée de terre froide. Le ver, lui, a déménagé.");
                    } else {
                        self.ver_trouve = true;
                        player.inventory.push(self.ver_id);
                        player.aura += 10000.0;
                        crate::audio::play_sound("assets/victory.wav");
                        println!("{} Vous dénichez un gros ver de terre bien gras. On ne sait jamais, ça peut servir.", colore!(Vert, "[+10 000 Aura]"));
                    }
                } else {
                    player.aura -= 15000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Vous vous mettez de la terre dans l'œil. Félicitations.", colore!(Rouge, "[-15 000 Aura]"));
                }
            }
            _ => println!("Action impossible sur la tombe."),
        }
    }
}
