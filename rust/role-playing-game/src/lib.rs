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
            (0, l) => Some(Player {
                health: REV_HEALTH,
                mana: (l >= REV_LVL_THRESHOLD).then(|| REV_MANA),
                level: l,
            }),
            (_, _) => None,
        };
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        return match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    0
                } else {
                    self.mana = Some(mana - mana_cost);
                    mana_cost * SPELL_MULTIPLIER
                }
            }
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        };
    }
}
