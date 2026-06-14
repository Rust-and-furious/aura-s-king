use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;

pub struct Roi {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub oeuf_id: usize,
    pub talisman_id: usize,
    pub bidule_id: usize,
    pub marmite_id: usize,
    pub seau_id: usize,
}

// la fin de partie est portée par WorldManager.fin_partie, pas par un état du Roi
impl Saveable for Roi {}

impl Interactable for Roi {
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
                println!("Le Roi Anthony vous toise depuis son trône légèrement de travers. Il a un besoin viscéral d'être impressionné.");
            }
            Action::Dialoguer => {
                let sujets = vec![
                    MenuOption::new("Se présenter (ÉVALUATION FINALE)"),
                    MenuOption::new("Offrir un objet"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Devant le Roi Anthony") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 5;
                        if player.aura >= 1_000_000.0 {
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{}", colore!(JauneGras, "Le Roi se lève, ébloui, et vous adoube sur-le-champ : « Relevez-vous, CHEVALIER ! » La foule applaudit, les poules et l'épouvantail aussi."));
                            println!("{}", colore!(VertGras, "=== VICTOIRE ABSOLUE ==="));
                            world.fin_partie = Some(true);
                        } else if jet_reussite(world.current_tick, 20) {
                            crate::audio::play_sound("assets/victory.wav");
                            println!("Le roi hésite... mais votre culot légendaire lui plaît. Il vous adoube sur un coup de tête !");
                            println!("{}", colore!(VertGras, "=== VICTOIRE DE JUSTESSE ==="));
                            world.fin_partie = Some(true);
                        } else {
                            crate::audio::play_sound("assets/defeat.wav");
                            println!("Le roi éclate de rire et vous fait jeter dehors. « Retourne sarcler tes navets, manant ! »");
                            println!("{}", colore!(RougeGras, "=== GAME OVER ==="));
                            world.fin_partie = Some(false);
                        }
                    }
                    1 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.oeuf_id) {
                            player.inventory.retain(|&x| x != self.oeuf_id);
                            player.aura += 250000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} Le roi adore l'œuf doré ! Il l'installe aussitôt sur son trône.", colore!(Vert, "[+250 000 Aura]"));
                        } else if player.inventory.contains(&self.talisman_id) {
                            player.inventory.retain(|&x| x != self.talisman_id);
                            player.aura += 100000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Un talisman ! Mystique, j'adore. »", colore!(Vert, "[+100 000 Aura]"));
                        } else if player.inventory.contains(&self.bidule_id) {
                            player.inventory.retain(|&x| x != self.bidule_id);
                            player.aura += 50000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « C'est moche... j'adore ! » dit le roi.", colore!(Vert, "[+50 000 Aura]"));
                        } else if player.inventory.contains(&self.marmite_id) {
                            player.inventory.retain(|&x| x != self.marmite_id);
                            player.aura -= 80000.0;
                            crate::audio::play_sound("assets/defeat.wav");
                            println!("{} « Des gardes ! Pourquoi m'offre-t-on un récipient à soupe ?! »", colore!(Rouge, "[-80 000 Aura]"));
                        } else if player.inventory.contains(&self.seau_id) {
                            player.inventory.retain(|&x| x != self.seau_id);
                            player.aura -= 200000.0;
                            crate::audio::play_sound("assets/defeat.wav");
                            println!("{} Le roi prend le seau vide pour une insulte royale. Très mauvaise idée.", colore!(Rouge, "[-200 000 Aura]"));
                        } else {
                            println!("Le roi attend manifestement quelque chose de plus impressionnant.");
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("Le Roi Anthony vous ignore avec une majesté étudiée."),
        }
    }
}
