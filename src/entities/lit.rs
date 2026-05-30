use crate::actions::Action;
use crate::entities::{Interactable, pseudo_rand};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;

pub struct Lit {
    pub name: String,
    pub description: String,
}

impl Useable for Lit {
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        let seed = world.current_tick;
        let elapsed = 60 + (pseudo_rand(seed) % 61);
        world.current_tick += elapsed;

        println!("Vous vous endormez pour {} minutes...", elapsed);

        let chance = pseudo_rand(seed + 1) % 100;
        if chance < 50 {
            player.aura += 10000.0;
            println!(
                "[+10 000 Aura] Rêve héroïque : vous vous voyez en armure étincelante sur un blanc destrier."
            );
        } else {
            player.aura -= 30000.0;
            println!(
                "[-30 000 Aura] Cauchemar de paysan : vous rêvez que vous êtes un chou cultivé et récolté par vous-même."
            );
        }
        Ok(())
    }
}

impl Interactable for Lit {
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
                println!("Vous observez le lit. {}", self.description);
                println!(
                    "*Note* : Vous y avez déjà passé trop de temps, voulez-vous vraiment rater votre vie ?"
                );
            }
            Action::Utiliser => {
                if let Err(e) = self.utiliser(player, world) {
                    println!("{}", e);
                }
            }
            _ => println!("Action impossible sur le lit."),
        }
    }
}
