const REV_HEALTH: u32 = 100;
const REV_LVL_THRESHOLD: u32 = 10;
const REV_MANA: u32 = 100;
const SPELL_MULTIPLIER: u32 = 2;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        return match (self.health, self.level) {
            (0,l) if l >= REV_LVL_THRESHOLD => Some(Player { health: REV_HEALTH, mana: Some(REV_MANA), level: l }),
            (0,l) => Some(Player { health: REV_HEALTH, mana: None, level: l }),
            (_,_) => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        return match (self.health, self.mana) {
            (_,Some(mana)) if mana < mana_cost => 0,
            (_,Some(mana)) => {
                self.mana = Some(mana - mana_cost);
                mana_cost * SPELL_MULTIPLIER
            },
            (health,_) if health >= mana_cost => {
                self.health -= mana_cost;
                0
            },
            (_,_) => {
                self.health = 0;
                0
            }
        }
    }
}
