use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Lac {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub piece_id: usize,
    pub deja_baigne: bool,
    pub deja_bu: bool,
}

impl Saveable for Lac {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("deja_baigne".to_string(), serde_json::json!(self.deja_baigne));
        state.insert("deja_bu".to_string(), serde_json::json!(self.deja_bu));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("deja_baigne") {
            if let Some(b) = val.as_bool() {
                self.deja_baigne = b;
            }
        }
        if let Some(val) = state.get("deja_bu") {
            if let Some(b) = val.as_bool() {
                self.deja_bu = b;
            }
        }
    }
}

impl Useable for Lac {
    // trois façons d'utiliser le lac → sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Se baigner"),
            MenuOption::new("Boire l'eau"),
            MenuOption::new("Jeter un objet"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "Le lac") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 30;
                // se sentir propre ne rapporte qu'une fois (sinon aura farmable à l'infini)
                if self.deja_baigne {
                    println!("Vous êtes déjà tout propre. Vous re-tremper ne ferait que vous refroidir.");
                } else {
                    self.deja_baigne = true;
                    player.aura += 15000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Bain rafraîchissant : vous sentez (un peu) moins le chou pour l'instant.", colore!(Vert, "[+15 000 Aura]"));
                }
            }
            1 => {
                world.current_tick += 3;
                if self.deja_bu {
                    println!("Vous n'avez plus soif. Vous forcer ne vous donnerait qu'une crampe.");
                } else {
                    self.deja_bu = true;
                    player.aura += 5000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} L'eau est fraîche et pure. Vous vous sentez revigoré.", colore!(Vert, "[+5 000 Aura]"));
                }
            }
            2 => {
                if player.inventory.is_empty() {
                    println!("Vous n'avez rien à jeter.");
                    return Ok(());
                }
                // on liste l'inventaire pour choisir l'objet à jeter
                let mut inv_opts: Vec<MenuOption> = player
                    .inventory
                    .iter()
                    .map(|&id| MenuOption::new(world.entities[id].name()))
                    .collect();
                inv_opts.push(MenuOption::special("Annuler"));
                let sel = match select_from_menu(inv_opts, "Jeter quel objet dans le lac ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return Ok(()),
                };
                if sel >= player.inventory.len() {
                    return Ok(());
                }
                world.current_tick += 3;
                let jete = player.inventory[sel];
                let nom = world.entities[jete].name().to_string();
                player.inventory.remove(sel);
                if jete == self.piece_id {
                    player.aura += 80000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} L'eau brille, une voix murmure : « Merci, ça faisait longtemps. »", colore!(Vert, "[+80 000 Aura]"));
                } else {
                    player.aura -= 25000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Vous jetez {} dans le lac. Plouf. Ça coule. Bravo, vous polluez la nature.", colore!(Rouge, "[-25 000 Aura]"), nom);
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Lac {
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
        vec![Action::Observer, Action::Utiliser]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Vous regardez votre reflet. Vous voyez un paysan. Mais si vous plissez les yeux... non, c'est toujours un paysan.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            _ => println!("Action impossible avec le lac."),
        }
    }
}
