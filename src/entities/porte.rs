use crate::actions::Action;
use crate::entities::{Interactable, pseudo_rand};
use crate::player::Player;
use crate::traits::{Fightable, Openable};
use crate::world::WorldManager;

pub struct Porte {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub est_ouverte: bool,
    pub is_locked: bool,
    pub key_entity_id: usize,
    pub target_zone: usize,
}

impl Openable for Porte {
    fn ouvrir(&mut self) -> Result<(), &'static str> {
        if self.is_locked {
            return Err("La porte est verrouillée à double tour.");
        }
        if self.est_ouverte {
            return Err("La porte est déjà ouverte.");
        }
        self.est_ouverte = true;
        Ok(())
    }

    fn fermer(&mut self) -> Result<(), &'static str> {
        if !self.est_ouverte {
            return Err("La porte est déjà fermée.");
        }
        self.est_ouverte = false;
        Ok(())
    }
}

impl Fightable for Porte {
    fn recevoir_degats(&mut self, _degats: i32) {
        // TODO : rajouter point de vie porte + logique ouverte quand pv à 0
    }

    fn est_vivant(&self) -> bool {
        self.is_locked
    }
}

impl Interactable for Porte {
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
        let mut actions = vec![
            Action::Observer,
            Action::Attaquer { degats: 10 },
            Action::Deplacer {
                target_zone: self.target_zone,
            },
        ];
        if self.est_ouverte {
            actions.push(Action::Fermer);
        } else {
            actions.push(Action::Ouvrir);
        }
        actions
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                println!("Vous observez la porte. {}", self.description);
                if self.est_ouverte {
                    println!("Elle est grande ouverte.");
                } else if self.is_locked {
                    println!("Elle est fermée et solidement verrouillée.");
                } else {
                    println!("Elle est fermée mais non verrouillée.");
                }
            }
            Action::Ouvrir => {
                if self.is_locked {
                    let has_key = player.inventory.contains(&self.key_entity_id);
                    if has_key {
                        self.is_locked = false;
                        self.est_ouverte = true;
                        world.current_tick += 1;
                        crate::audio::play_sound("assets/victory.wav");
                        println!(
                            "Vous insérez la clé de la maison dans la serrure. Le loquet cède avec un clic satisfaisant. La porte s'ouvre !"
                        );
                    } else {
                        player.aura -= 5000.0;
                        crate::audio::play_sound("assets/defeat.wav");
                        println!(
                            "\n\x1B[31m[-5 000 Aura]\x1B[0m Vous poussez. Rien. Vous repoussez. Toujours rien. Humiliant. Même un âne mourant aurait fait mieux."
                        );
                    }
                } else {
                    match self.ouvrir() {
                        Ok(_) => {
                            world.current_tick += 1;
                            println!("Vous ouvrez la porte.");
                        }
                        Err(e) => println!("{}", e),
                    }
                }
            }
            Action::Fermer => match self.fermer() {
                Ok(_) => {
                    world.current_tick += 1;
                    println!("Vous fermez la porte.");
                }
                Err(e) => println!("{}", e),
            },
            Action::Attaquer { degats: _ } => {
                world.current_tick += 5;
                let seed = world.current_tick;

                if self.is_locked {
                    let chance = pseudo_rand(seed) % 100;
                    if chance < 50 {
                        self.is_locked = false;
                        self.est_ouverte = true;
                        player.aura += 25000.0;
                        crate::audio::play_sound("assets/victory.wav");
                        println!(
                            "\n\x1B[32m[+25 000 Aura]\x1B[0m HÉROÏQUE ! D'un coup d'épaule phénoménal, vous enfoncez la porte ! Le chambranle vole en éclats !"
                        );
                    } else {
                        player.aura -= 15000.0;
                        crate::audio::play_sound("assets/defeat.wav");
                        println!(
                            "\n\x1B[31m[-15 000 Aura]\x1B[0m AÏE ! Vous vous jetez sur la porte en bois massif. La porte ne bouge pas d'un millimètre, votre épaule si. Elle est légèrement démise."
                        );
                    }
                } else {
                    self.est_ouverte = true;
                    player.aura -= 20000.0;
                    crate::audio::play_sound("assets/defeat.wav");
                    println!(
                        "\n\x1B[31m[-20 000 Aura]\x1B[0m Vous enfoncez une porte ouverte. Littéralement. Vous trébuchez et tombez à plat ventre dans la poussière. Tout le monde vous regarde bizarrement."
                    );
                }
            }
            Action::Deplacer { target_zone } => {
                if self.est_ouverte {
                    world.current_tick += 10;
                    player.zone = *target_zone;
                    println!("Vous passez la porte et sortez de chez vous.");
                } else {
                    println!(
                        "La porte est fermée ! Vous ne pouvez pas passer à travers (à moins de l'enfoncer)."
                    );
                }
            }
            _ => println!("Action impossible sur la porte."),
        }
    }
}
