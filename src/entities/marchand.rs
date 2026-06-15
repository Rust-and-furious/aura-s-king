use crate::actions::Action;
use crate::entities::{Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::world::WorldManager;

pub struct Marchand {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub armure_id: usize,
    pub rubis_id: usize,
    pub piece_id: usize,
}

impl Saveable for Marchand {}

impl Interactable for Marchand {
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
            // boutique → sous-menu d'achats/échanges
            Action::Dialoguer => {
                let sujets = vec![
                    MenuOption::new("Acheter le Philtre de Charisme Absolu (1 pièce)"),
                    MenuOption::new("Échanger le Rubis Rutilant"),
                    MenuOption::new("Parler"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "L'étal du marchand") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.piece_id) {
                            player.inventory.retain(|&x| x != self.piece_id);
                            // pas d'inventaire « actif » dans le moteur : on boit le philtre à l'achat (c'est un piège)
                            player.aura -= 50000.0;
                            world.current_tick += 60;
                            crate::audio::play_sound("assets/defeat.wav");
                            println!("{} « Un Philtre de Charisme Absolu ! » Vous le buvez cul sec... C'était de l'eau du lac et du jus de chou. Vous êtes malade comme un chien pendant une heure.", colore!(Rouge, "[-50 000 Aura]"));
                        } else {
                            println!("« Pas de pièce, pas de philtre. Reviens quand tu seras solvable. »");
                        }
                    }
                    1 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.rubis_id) {
                            player.inventory.retain(|&x| x != self.rubis_id);
                            player.inventory.push(self.armure_id);
                            player.aura += 500000.0;
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Par tous les dieux, un rubis rutilant ! Tiens, prends cette armure rutilante, elle est digne d'un roi. » Vous l'enfilez : vous en jetez ENFIN.", colore!(Vert, "[+500 000 Aura]"));
                        } else {
                            println!("« Tu n'as rien qui vaille mon armure. Reviens avec quelque chose qui brille. »");
                        }
                    }
                    2 => {
                        world.current_tick += 3;
                        println!("« J'ai de tout : des philtres, des rumeurs, et des regrets en promotion. »");
                    }
                    _ => {}
                }
            }
            _ => println!("Le marchand range discrètement ses objets de valeur."),
        }
    }
}
