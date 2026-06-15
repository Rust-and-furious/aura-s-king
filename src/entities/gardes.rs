use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::menu::{select_from_menu, MenuOption, MenuResult};
use crate::player::Player;
use crate::traits::Fightable;
use crate::world::WorldManager;

pub struct Gardes {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub cape_id: usize,
    pub armure_id: usize,
    pub piece_id: usize,
    pub rubis_id: usize,
    pub laissez_passer_id: usize,
}

// pas d'état propre : l'accès accordé est matérialisé par le laissez-passer dans l'inventaire
impl Saveable for Gardes {}

impl Gardes {
    // accorde l'accès au château en donnant le laissez-passer (une seule fois)
    fn accorder_acces(&self, player: &mut Player) {
        if !player.inventory.contains(&self.laissez_passer_id) {
            player.inventory.push(self.laissez_passer_id);
        }
    }
}

impl Fightable for Gardes {
    fn recevoir_degats(&mut self, _degats: i32) {}

    // les gardes sont bien vivants (hélas pour vous)
    fn est_vivant(&self) -> bool {
        true
    }
}

impl Interactable for Gardes {
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
        vec![Action::Observer, Action::Dialoguer, Action::Attaquer { degats: 10 }]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Les gardes ont l'air de s'ennuyer profondément. Mais ils sont costauds.");
            }
            Action::Dialoguer => {
                let sujets = vec![
                    MenuOption::new("Demander à entrer"),
                    MenuOption::new("Corrompre"),
                    MenuOption::new("Parler"),
                    MenuOption::special("Retour"),
                ];
                let choix = match select_from_menu(sujets, "Les gardes du château") {
                    Ok(MenuResult::Selected(i)) => i,
                    _ => return,
                };
                match choix {
                    0 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.laissez_passer_id) {
                            println!("« L'accès vous est déjà accordé, noble personne. Entrez donc. »");
                        } else if player.inventory.contains(&self.cape_id)
                            || player.inventory.contains(&self.armure_id)
                        {
                            self.accorder_acces(player);
                            crate::audio::play_sound("assets/victory.wav");
                            println!("« Tenue correcte exigée... et respectée. Vous pouvez passer. »");
                        } else {
                            println!("« Reviens quand t'auras l'air de quelqu'un d'important. »");
                        }
                    }
                    1 => {
                        world.current_tick += 5;
                        if player.inventory.contains(&self.laissez_passer_id) {
                            println!("Vous avez déjà l'accès. Inutile de gaspiller votre or.");
                        } else if player.inventory.contains(&self.piece_id) {
                            player.inventory.retain(|&x| x != self.piece_id);
                            player.aura += 20000.0;
                            self.accorder_acces(player);
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Hé, glisse-nous une pièce et on n'a rien vu. » Affaire conclue.", colore!(Vert, "[+20 000 Aura]"));
                        } else if player.inventory.contains(&self.rubis_id) {
                            player.inventory.retain(|&x| x != self.rubis_id);
                            player.aura += 20000.0;
                            self.accorder_acces(player);
                            crate::audio::play_sound("assets/victory.wav");
                            println!("{} « Un rubis ?! Euh... entrez, entrez, votre Altesse ! »", colore!(Vert, "[+20 000 Aura]"));
                        } else {
                            println!("« T'as rien pour nous convaincre. Circule. »");
                        }
                    }
                    2 => {
                        world.current_tick += 3;
                        println!("« On garde. C'est notre truc. On est gardes. »");
                    }
                    _ => {}
                }
            }
            Action::Attaquer { degats } => {
                self.recevoir_degats(*degats);
                world.current_tick += 15;
                if player.inventory.contains(&self.laissez_passer_id) {
                    println!("Vous avez déjà l'accès. Inutile de jouer les héros.");
                } else if jet_reussite(world.current_tick, 10) {
                    player.aura += 150000.0;
                    self.accorder_acces(player);
                    crate::audio::play_sound("assets/victory.wav");
                    println!("{} Dans un éclair de bravoure, vous forcez le passage ! Les gardes, sonnés, vous laissent entrer.", colore!(Vert, "[+150 000 Aura]"));
                } else {
                    world.current_tick += 60;
                    player.aura -= 100000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!("{} Les gardes vous plaquent au sol en 0,3 seconde. Votre visage goûte la poussière pendant votre garde à vue.", colore!(Rouge, "[-100 000 Aura]"));
                }
            }
            _ => println!("Action impossible sur les gardes."),
        }
    }
}
