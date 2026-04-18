#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombatResult {
    PlayerWins { new_force: i32 },
    PlayerDies,
}

pub fn resolve(player_force: i32, enemy_force: i32) -> CombatResult {
    let new_force = player_force - enemy_force;
    if new_force <= 0 {
        CombatResult::PlayerDies
    } else {
        CombatResult::PlayerWins { new_force }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_wins() {
        assert_eq!(resolve(5, 3), CombatResult::PlayerWins { new_force: 2 });
    }
    #[test]
    fn player_dies_negative() {
        assert_eq!(resolve(5, 7), CombatResult::PlayerDies);
    }
    #[test]
    fn player_dies_at_zero() {
        assert_eq!(resolve(5, 5), CombatResult::PlayerDies);
    }
}
