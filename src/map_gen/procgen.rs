use super::room::{GridPos, RoomLayout, TileKind, GRID_WIDTH, GRID_HEIGHT};
use crate::enemies::components::{EnemyDef, EnemyBehavior, Axis};
use crate::items::components::{ItemDef, ItemKind};
use std::collections::VecDeque;

/// LCG sub-seed mixer. Unique per (run, floor, salt) triple.
pub fn sub_seed(run_seed: u64, floor: u8, salt: u64) -> u64 {
    run_seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(floor as u64)
        .wrapping_add(salt)
}

/// Fisher-Yates shuffle using sub_seed as the PRNG state.
fn shuffle_candidates(mut candidates: Vec<GridPos>, seed: u64) -> Vec<GridPos> {
    let mut state = seed;
    let n = candidates.len();
    for i in (1..n).rev() {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = (state >> 33) as usize % (i + 1);
        candidates.swap(i, j);
    }
    candidates
}

/// BFS connectivity check treating Wall tiles as impassable.
fn is_reachable(from: GridPos, to: GridPos, layout: &RoomLayout) -> bool {
    let mut visited = [[false; 8]; 8];
    let mut queue = VecDeque::new();
    queue.push_back(from);
    visited[from.y as usize][from.x as usize] = true;

    while let Some(cur) = queue.pop_front() {
        if cur == to {
            return true;
        }
        for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
            let nx = cur.x as i32 + dx;
            let ny = cur.y as i32 + dy;
            if !(0..8).contains(&nx) || !(0..8).contains(&ny) {
                continue;
            }
            let (nx, ny) = (nx as u8, ny as u8);
            if visited[ny as usize][nx as usize] {
                continue;
            }
            if layout.get(nx, ny) == TileKind::Wall {
                continue;
            }
            visited[ny as usize][nx as usize] = true;
            queue.push_back(GridPos { x: nx, y: ny });
        }
    }

    false
}

/// Pick exit position from interior-border candidates (18 total, excluding player spawn (1,1)).
/// Interior-border = tiles on the inner edge of the walkable area (x or y in {1,6}).
fn place_exit(floor: u8, run_seed: u64) -> GridPos {
    let candidates = [
        GridPos { x: 1, y: 2 }, GridPos { x: 1, y: 3 }, GridPos { x: 1, y: 4 }, GridPos { x: 1, y: 5 }, GridPos { x: 1, y: 6 },
        GridPos { x: 6, y: 2 }, GridPos { x: 6, y: 3 }, GridPos { x: 6, y: 4 }, GridPos { x: 6, y: 5 }, GridPos { x: 6, y: 6 },
        GridPos { x: 2, y: 1 }, GridPos { x: 3, y: 1 }, GridPos { x: 4, y: 1 }, GridPos { x: 5, y: 1 },
        GridPos { x: 2, y: 6 }, GridPos { x: 3, y: 6 }, GridPos { x: 4, y: 6 }, GridPos { x: 5, y: 6 },
    ];

    let seed = sub_seed(run_seed, floor, 0);
    let idx = (seed as usize) % candidates.len();
    candidates[idx]
}

/// Place 0–3 wall (pillar) tiles inside the 3x3 center grid (2,2)–(4,4).
/// Validates BFS connectivity from spawn (1,1) to exit after each placement.
fn place_pillars(
    floor: u8,
    run_seed: u64,
    layout: &mut RoomLayout,
    exit_pos: GridPos,
) -> Vec<GridPos> {
    let pillar_candidates = vec![
        GridPos { x: 2, y: 2 }, GridPos { x: 3, y: 2 }, GridPos { x: 4, y: 2 },
        GridPos { x: 2, y: 3 }, GridPos { x: 3, y: 3 }, GridPos { x: 4, y: 3 },
        GridPos { x: 2, y: 4 }, GridPos { x: 3, y: 4 }, GridPos { x: 4, y: 4 },
    ];

    let count = (sub_seed(run_seed, floor, 1) % 4) as usize;
    let shuffled = shuffle_candidates(pillar_candidates, sub_seed(run_seed, floor, 2));

    let mut placed = Vec::new();
    for candidate in shuffled.iter().take(count) {
        layout.set(candidate.x, candidate.y, TileKind::Wall);
        if is_reachable(GridPos { x: 1, y: 1 }, exit_pos, layout) {
            placed.push(*candidate);
        } else {
            layout.set(candidate.x, candidate.y, TileKind::Floor);
        }
    }

    placed
}

/// Main entry point: builds a full room layout procedurally.
/// Floor 10 returns an open arena with no exit and no pillars.
pub fn build_room_proc(floor: u8, run_seed: u64) -> (RoomLayout, GridPos) {
    let mut layout = [[TileKind::Floor; 8]; 8];

    for x in 0..GRID_WIDTH {
        layout[0][x as usize] = TileKind::Wall;
        layout[7][x as usize] = TileKind::Wall;
    }
    for y in 0..GRID_HEIGHT {
        layout[y as usize][0] = TileKind::Wall;
        layout[y as usize][7] = TileKind::Wall;
    }

    if floor == 10 {
        // Open boss arena: no exit, no pillars
        return (RoomLayout::new(layout), GridPos { x: 0, y: 0 });
    }

    let exit_pos = place_exit(floor, run_seed);
    layout[exit_pos.y as usize][exit_pos.x as usize] = TileKind::Exit;

    let mut room_layout = RoomLayout::new(layout);
    let _pillars = place_pillars(floor, run_seed, &mut room_layout, exit_pos);

    (room_layout, exit_pos)
}

/// Generate 2–4 enemy definitions procedurally.
pub fn generate_enemy_defs(
    floor: u8,
    run_seed: u64,
    layout: &RoomLayout,
    exit_pos: GridPos,
) -> Vec<EnemyDef> {
    let count = 2 + (sub_seed(run_seed, floor, 10) % 3) as usize;
    let player_spawn = GridPos { x: 1, y: 1 };

    let candidates: Vec<GridPos> = (1u8..7)
        .flat_map(|y| (1u8..7).map(move |x| GridPos { x, y }))
        .filter(|p| {
            layout.get(p.x, p.y) == TileKind::Floor
                && *p != player_spawn
                && *p != exit_pos
        })
        .collect();

    let shuffled = shuffle_candidates(candidates, sub_seed(run_seed, floor, 11));
    let base_force = floor as i32 + 2;

    shuffled
        .into_iter()
        .take(count)
        .enumerate()
        .map(|(i, pos)| {
            let force = base_force + (i as i32 * 2);
            let behavior = match i % 3 {
                0 => EnemyBehavior::Guard {
                    alerted: false,
                },
                1 => EnemyBehavior::Patrol {
                    axis: Axis::Horizontal,
                    direction: 1,
                },
                _ => EnemyBehavior::Static,
            };
            EnemyDef { pos, force, behavior }
        })
        .collect()
}

/// Generate 1–2 item definitions procedurally, avoiding spawn, exit, walls, and enemy positions.
pub fn generate_item_defs(
    floor: u8,
    run_seed: u64,
    layout: &RoomLayout,
    exit_pos: GridPos,
    enemy_positions: &[GridPos],
) -> Vec<ItemDef> {
    let count = 1 + (sub_seed(run_seed, floor, 200) % 2) as usize;
    let player_spawn = GridPos { x: 1, y: 1 };

    let candidates: Vec<GridPos> = (1u8..7)
        .flat_map(|y| (1u8..7).map(move |x| GridPos { x, y }))
        .filter(|p| {
            layout.get(p.x, p.y) == TileKind::Floor
                && *p != player_spawn
                && *p != exit_pos
                && !enemy_positions.contains(p)
        })
        .collect();

    let shuffled = shuffle_candidates(candidates, sub_seed(run_seed, floor, 201));

    shuffled
        .into_iter()
        .take(count)
        .enumerate()
        .map(|(i, pos)| {
            let kind = match sub_seed(run_seed, floor, 202 + i as u64) % 3 {
                0 => ItemKind::Ration,
                1 => ItemKind::Whetstone,
                _ => ItemKind::Runa,
            };
            ItemDef { pos, kind }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism_same_seed() {
        let (layout_a, exit_a) = build_room_proc(1, 0xDEADBEEF_CAFEBABE);
        let (layout_b, exit_b) = build_room_proc(1, 0xDEADBEEF_CAFEBABE);
        assert_eq!(exit_a, exit_b);
        for y in 0..8u8 {
            for x in 0..8u8 {
                assert_eq!(
                    layout_a.get(x, y),
                    layout_b.get(x, y),
                    "Tile mismatch at ({},{})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_different_seeds_produce_different_exits() {
        let exits: Vec<_> = (0u64..20)
            .map(|s| {
                let (_, exit) = build_room_proc(1, s.wrapping_mul(1234567891011));
                exit
            })
            .collect();
        let unique: std::collections::HashSet<_> = exits.iter().collect();
        assert!(
            unique.len() > 3,
            "Expected variety in exit positions, got {:?}",
            unique
        );
    }

    #[test]
    fn test_exit_not_on_player_spawn() {
        for seed in [0u64, 1, 42, 999, u64::MAX, u64::MAX / 2] {
            for floor in 1u8..=10 {
                let (_, exit) = build_room_proc(floor, seed);
                assert_ne!(
                    exit,
                    GridPos { x: 1, y: 1 },
                    "Exit landed on player spawn for seed={} floor={}",
                    seed,
                    floor
                );
            }
        }
    }

    #[test]
    fn test_room_always_connected() {
        for seed in [0u64, 1, 7, 42, 12345, 999999, u64::MAX] {
            for floor in 1u8..=9 {
                let (layout, exit) = build_room_proc(floor, seed);
                assert!(
                    is_reachable(GridPos { x: 1, y: 1 }, exit, &layout),
                    "Room disconnected: seed={} floor={}",
                    seed,
                    floor
                );
            }
        }
    }

    #[test]
    fn test_perimeter_always_walls() {
        let (layout, _) = build_room_proc(3, 0xABCD1234);
        for i in 0..8u8 {
            assert_eq!(layout.get(i, 0), TileKind::Wall);
            assert_eq!(layout.get(i, 7), TileKind::Wall);
            assert_eq!(layout.get(0, i), TileKind::Wall);
            assert_eq!(layout.get(7, i), TileKind::Wall);
        }
    }

    #[test]
    fn test_item_positions_avoid_walls_spawn_exit_enemies() {
        let (layout, exit) = build_room_proc(1, 0xFEEDFACE);
        let enemy_defs = generate_enemy_defs(1, 0xFEEDFACE, &layout, exit);
        let enemy_positions: Vec<GridPos> = enemy_defs.iter().map(|d| d.pos).collect();
        let items = generate_item_defs(1, 0xFEEDFACE, &layout, exit, &enemy_positions);
        let player_spawn = GridPos { x: 1, y: 1 };
        for item in &items {
            assert_ne!(item.pos, player_spawn, "Item on player spawn");
            assert_ne!(item.pos, exit, "Item on exit");
            assert_ne!(layout.get(item.pos.x, item.pos.y), TileKind::Wall, "Item on wall");
            assert!(!enemy_positions.contains(&item.pos), "Item on enemy");
        }
        assert!(items.len() >= 1 && items.len() <= 2);
    }

    #[test]
    fn test_enemy_positions_avoid_walls_spawn_exit() {
        let (layout, exit) = build_room_proc(2, 0xFEEDFACE);
        let defs = generate_enemy_defs(2, 0xFEEDFACE, &layout, exit);
        let player_spawn = GridPos { x: 1, y: 1 };
        for def in &defs {
            assert_ne!(def.pos, player_spawn, "Enemy on player spawn");
            assert_ne!(def.pos, exit, "Enemy on exit");
            assert_ne!(
                layout.get(def.pos.x, def.pos.y),
                TileKind::Wall,
                "Enemy on wall"
            );
        }
        assert!(defs.len() >= 2 && defs.len() <= 4);
    }

    #[test]
    fn test_sub_seed_uniqueness() {
        let s0 = sub_seed(42, 1, 0);
        let s1 = sub_seed(42, 1, 1);
        let s2 = sub_seed(42, 2, 0);
        assert_ne!(s0, s1);
        assert_ne!(s0, s2);
    }

    #[test]
    fn test_floor_progression_variety() {
        let seed = 0x0123456789ABCDEF;
        let exits: Vec<_> = (1u8..=9)
            .map(|f| {
                let (_, e) = build_room_proc(f, seed);
                e
            })
            .collect();
        let first = exits[0];
        assert!(
            exits.iter().any(|e| *e != first),
            "All floors produced the same exit — seed mixing may be broken"
        );
    }
}
