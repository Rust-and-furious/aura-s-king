use crate::actions::Action;
use crate::entities::{jet_reussite, Interactable, Saveable};
use crate::player::Player;
use crate::traits::Useable;
use crate::world::WorldManager;

// champignon suspect : se mange (gros pari 50/50) ou se cueille (objet d'échange). Consommé dans les deux cas.
pub struct Champignon {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub champignon_objet_id: usize,
}

impl Saveable for Champignon {}

impl Useable for Champignon {
    // manger le champignon : 50/50, puis il est consommé (plus rien à manger ni à cueillir)
    fn utiliser(
        &mut self,
        player: &mut Player,
        world: &mut WorldManager,
    ) -> Result<(), &'static str> {
        if jet_reussite(world.current_tick, 50) {
            world.current_tick += 10;
            player.aura += 100000.0;
            crate::audio::play_sound("assets/victory.wav");
            println!("{} Vision mystique ! Vous percevez les secrets de l'univers (et le vrai sens du mot 'navet').", colore!(Vert, "[+100 000 Aura]"));
        } else {
            world.current_tick += 90;
            player.aura -= 50000.0;
            crate::audio::play_sound("assets/defeat.wav");
            println!("{} Intoxication. Vous parlez aux arbres. Ils ne répondent pas. Vous vomissez votre chou et reprenez vos esprits dans une mare.", colore!(Rouge, "[-50 000 Aura]"));
        }
        // consommé quel que soit le résultat
        world.remove_interactable_from_zone(player.zone, self.id);
        Ok(())
    }
}

impl Interactable for Champignon {
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
        vec![Action::Observer, Action::Utiliser, Action::Ramasser]
    }

    fn execute_action(&mut self, action: &Action, player: &mut Player, world: &mut WorldManager) {
        match action {
            Action::Observer => {
                world.current_tick += 1;
                println!("Il est violet, brillant et vibre légèrement. Tout va bien.");
            }
            Action::Utiliser => {
                let _ = self.utiliser(player, world);
            }
            Action::Ramasser => {
                world.current_tick += 3;
                // cueilli sans le manger : on le garde pour un éventuel échange
                world.remove_interactable_from_zone(player.zone, self.id);
                player.inventory.push(self.champignon_objet_id);
                println!("Vous cueillez délicatement le champignon et le glissez dans votre besace.");
            }
            _ => println!("Action impossible sur le champignon."),
        }
    }
}
