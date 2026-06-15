use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;
use std::collections::HashMap;

pub struct Ermite {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub medaille_id: usize,
    pub talisman_id: usize,
    pub champignon_id: usize,
    pub marmite_id: usize,
    pub conseil_donne: bool,
}

impl Saveable for Ermite {
    fn save_state(&self) -> HashMap<String, serde_json::Value> {
        let mut state = HashMap::new();
        state.insert("conseil_donne".to_string(), serde_json::json!(self.conseil_donne));
        state
    }

    fn load_state(&mut self, state: &HashMap<String, serde_json::Value>) {
        if let Some(val) = state.get("conseil_donne") {
            if let Some(b) = val.as_bool() {
                self.conseil_donne = b;
            }
        }
    }
}

impl Interactable for Ermite {
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
        vec![Action::Observer, Action::Dialoguer]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("{}", self.description);
            }
            // plusieurs sujets de conversation → on ouvre un sous-menu
            Action::Dialoguer => {
                let sujets = vec![
                    MenuOption::new("Parler"),
                    MenuOption::new("Demander conseil"),
                    MenuOption::new("Offrir un objet"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Que dire à l'ermite ?") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 5;
                        println!("« Mmh ? Un paysan ? Je suis un ancien chevalier. J'ai tout quitté pour vivre dans cet arbre. Meilleure décision de ma vie. »");
                    }
                    1 => {
                        world.current_tick += 5;
                        // un conseil utile ne se monnaie qu'une fois en aura
                        if self.conseil_donne {
                            println!("« Je te l'ai déjà dit : aie l'air sûr de toi. Tu vas finir par m'agacer. »");
                        } else {
                            self.conseil_donne = true;
                            player.aura += 10000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Règle numéro un : aie toujours l'air sûr de toi, même quand tu ne sais pas ce que tu fais. Surtout quand tu ne sais pas. »", colore!(Vert, "[+10 000 Aura]"));
                        }
                    }
                    2 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.champignon_id) {
                            player.inventory.retain(|&x| x != self.champignon_id);
                            player.inventory.push(self.medaille_id);
                            player.aura += 80000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « AH ! Mon champignon ! Ça fait 12 ans que j'en cherche ! Tiens, prends cette médaille. »", colore!(Vert, "[+80 000 Aura]"));
                        } else if player.inventory.contains(&self.marmite_id) {
                            player.inventory.retain(|&x| x != self.marmite_id);
                            player.inventory.push(self.talisman_id);
                            player.aura += 40000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Je peux en faire une casserole. Tiens, un talisman protecteur en échange. »", colore!(Vert, "[+40 000 Aura]"));
                        } else {
                            println!("« Non merci, la nature me fournit le nécessaire. »");
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("L'ermite vous ignore et caresse sa longue barbe."),
        }
    }
}
