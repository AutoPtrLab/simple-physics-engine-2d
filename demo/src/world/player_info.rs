//struct defininf the spells the player can cast and the hp and so on

pub struct PlayerInfo {
    pub max_hp: i16, //the maximum health of the player
    pub hp: i16,     //the current hp of the player
    pub gold: u16,
}
impl Default for PlayerInfo {
    fn default() -> Self {
        let max_hp: i16 = 10;
        Self {
            max_hp,
            hp: max_hp,
            gold: 5000,
        }
    }
}
