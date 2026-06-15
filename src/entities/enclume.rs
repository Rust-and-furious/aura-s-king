use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Fightable;
use crate::traits::Useable;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Enclume {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub bidule_id: usize,
    pub marmite_id: usize,
    pub bidule_pris: bool,
}

impl Saveable for Enclume {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("bidule_pris".to_string(), serde_json::json!(self.bidule_pris));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("bidule_pris") {
            if let Some(b) = val.as_bool() {
                self.bidule_pris = b;
            }
        }
    }
}

impl Fightable for Enclume {
    // une enclume n'a pas de PV : on tape dessus, ça ne la "tue" pas
    fn recevoir_degats(&mut self, _degats: i32) {}

    fn est_vivant(&self) -> bool {
        false
    }
}

impl Useable for Enclume {
    // utiliser la forge ou tenter l'exploit de force → sous-menu
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let options = vec![
            MenuOption::new("Demander à utiliser la forge"),
            MenuOption::new("Essayer de soulever l'enclume"),
            MenuOption::special("Retour"),
        ];
        let choix = match select_from_menu(options, "L'enclume du forgeron") {
            Ok(MenuResult::Selected(i)) => i,
            _ => return Ok(()),
        };
        match choix {
            0 => {
                world.current_tick += 30;
                if jet_reussite(world.current_tick, 30) {
                    // un seul bidule utile : ensuite on ne fait que de la ferraille sans valeur
                    if self.bidule_pris {
                        println!("Vous forgez un énième bidule informe. Vous n'en avez aucun usage de plus.");
                    } else {
                        self.bidule_pris = true;
                        player.inventory.push(self.bidule_id);
                        player.aura += 30000.0;
                        crate::audio::play_sound("assets/victory.wav");
                        println!("{} Vous martelez un bidule en métal informe mais solide. Le forgeron hoche la tête, à demi impressionné.", colore!(Vert, "[+30 000 Aura]"));
                    }
                } else {
                    player.aura -= 15000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Vous vous brûlez au second degré. Le forgeron soupire.", colore!(Rouge, "[-15 000 Aura]"));
                }
            }
            1 => {
                world.current_tick += 5;
                // 3 % seulement : au-delà, vu le jackpot (+900 000), l'espérance redevient positive et farmable
                if jet_reussite(world.current_tick, 3) {
                    player.aura += 900000.0;
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} L'EXPLOIT ! Vous soulevez l'enclume au-dessus de votre tête. Le village entier vous acclame en héros !", colore!(Vert, "[+900 000 Aura]"));
                } else {
                    player.aura -= 30000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Non. Votre colonne vertébrale refuse catégoriquement.", colore!(Rouge, "[-30 000 Aura]"));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Interactable for Enclume {
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
        vec![Action::Observer, Action::Utiliser, Action::Attaquer { degats: 1 }]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Lourde. Très lourde. Parfaitement enclume.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            Action::Attaquer { degats } => {
                self.recevoir_degats(*degats);
                world.current_tick += 3;
                if player.inventory.contains(&self.marmite_id) {
                    player.aura -= 15000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} BONG ! La marmite est cabossée. Le forgeron vous jette un regard noir.", colore!(Rouge, "[-15 000 Aura]"));
                } else {
                    println!("Vous frappez l'enclume à mains nues. Vos poignets désapprouvent vivement.");
                }
            }
            _ => println!("Action impossible sur l'enclume."),
        }
    }
}
