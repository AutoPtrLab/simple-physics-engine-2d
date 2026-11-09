//this system cleans up all the dead enemies torrests and bullets

use crate::world::level::Level;

pub fn clean_entities(level: &mut Level) {
    //we loop to find the dead enemies
    level.enemies.retain(|_id, e| {
        if e.hp == i16::MIN {
            level.bodies.remove(e.body);
            return false;
        }
        true
    });

    level.spells.retain(|_id, spell| {
        if spell.is_dead {
            level.bodies.remove(spell.body);
            return false;
        }
        true
    });
    level.obstacles.retain(|_id, obstacle| {
        if obstacle.durability <= 0 {
            level.bodies.remove(obstacle.body);
            return false;
        }
        true
    });
}
