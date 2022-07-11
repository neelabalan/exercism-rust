// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mana = match self.mana {
                Some(val) => Some(100),
                None => None
            };
            return Some(Player{ health: 100, mana: mana, level: self.level});
        }
        else {
            return None;
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mut return_val: u32 = 0;
        match self.mana {
            Some(val) => {
                if val > mana_cost {
                    return_val = 2 * mana_cost;
                    self.mana = Some(val - mana_cost);
                }
            },
            None => {
                if self.health > mana_cost {
                    self.health = self.health - mana_cost;
                }
                else {
                    self.health = 0;
                }
            }
        };
        return return_val;
    }
}
