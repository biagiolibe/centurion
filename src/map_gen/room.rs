use bevy::prelude::*;

pub const TILE_SIZE: f32 = 64.0;
pub const GRID_WIDTH: u8 = 8;
pub const GRID_HEIGHT: u8 = 8;

/// Position on the game grid (0..8 on both axes).
/// Grid origin (0,0) is top-left; X increases right, Y increases down (standard grid convention).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPos {
    pub x: u8,
    pub y: u8,
}

/// Type of tile in the room.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileKind {
    /// Normal passable tile.
    Floor,
    /// Solid wall, blocks movement.
    Wall,
    /// Exit to next floor.
    Exit,
}

/// The layout of a room: all tile types for the 8x8 grid.
/// Direct indexing: `layout.tiles[y][x]` where y is row, x is column.
#[derive(Resource, Clone)]
pub struct RoomLayout {
    tiles: [[TileKind; 8]; 8],
}

impl RoomLayout {
    /// Get the tile type at grid position (x, y).
    /// Returns Floor if out of bounds (defensive).
    pub fn get(&self, x: u8, y: u8) -> TileKind {
        if x < GRID_WIDTH && y < GRID_HEIGHT {
            self.tiles[y as usize][x as usize]
        } else {
            TileKind::Floor
        }
    }
}

/// Build a room layout for a given floor number.
/// No PRNG — layout is purely deterministic based on floor.
/// - All perimeter tiles are walls
/// - Interior is floor
/// - One exit tile at a position derived from floor number
pub fn build_room(floor: u8) -> RoomLayout {
    let mut layout = [[TileKind::Floor; 8]; 8];

    // Perimeter walls
    for x in 0..GRID_WIDTH {
        layout[0][x as usize] = TileKind::Wall;
        layout[7][x as usize] = TileKind::Wall;
    }
    for y in 0..GRID_HEIGHT {
        layout[y as usize][0] = TileKind::Wall;
        layout[y as usize][7] = TileKind::Wall;
    }

    // Exit tile: deterministic position based on floor
    // Floor 1: (4,4), Floor 2: (5,4), Floor 3: (6,4), Floor 4: (4,4) again, etc.
    let exit_x = 4 + ((floor.saturating_sub(1)) % 3);
    let exit_y = 4;
    if exit_x < GRID_WIDTH && exit_y < GRID_HEIGHT {
        layout[exit_y as usize][exit_x as usize] = TileKind::Exit;
    }

    RoomLayout { tiles: layout }
}

/// Convert grid position to world coordinates.
/// Grid (0,0) is top-left; world (0,0) is center.
pub fn grid_to_world(pos: GridPos) -> Vec2 {
    Vec2::new(
        (pos.x as f32 - 3.5) * TILE_SIZE,
        (3.5 - pos.y as f32) * TILE_SIZE,
    )
}

/// Convert world coordinates to grid position (with rounding).
pub fn world_to_grid(world_pos: Vec2) -> GridPos {
    let x = ((world_pos.x / TILE_SIZE) + 3.5).round().clamp(0.0, 7.0) as u8;
    let y = ((3.5 - (world_pos.y / TILE_SIZE))).round().clamp(0.0, 7.0) as u8;
    GridPos { x, y }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_to_world() {
        // Grid (0,0) top-left: x = (0 - 3.5)*64 = -224, y = (3.5 - 0)*64 = 224
        let top_left = GridPos { x: 0, y: 0 };
        let world = grid_to_world(top_left);
        assert_eq!(world.x, -3.5 * TILE_SIZE);
        assert_eq!(world.y, 3.5 * TILE_SIZE);

        // Grid (7,7) bottom-right: x = (7 - 3.5)*64 = 224, y = (3.5 - 7)*64 = -224
        let bottom_right = GridPos { x: 7, y: 7 };
        let world = grid_to_world(bottom_right);
        assert_eq!(world.x, 3.5 * TILE_SIZE);
        assert_eq!(world.y, -3.5 * TILE_SIZE);

        // Grid (3,3): x = (3 - 3.5)*64 = -32, y = (3.5 - 3)*64 = 32
        let near_center = GridPos { x: 3, y: 3 };
        let world = grid_to_world(near_center);
        assert_eq!(world.x, -0.5 * TILE_SIZE);
        assert_eq!(world.y, 0.5 * TILE_SIZE);

        // Grid (4,4): x = (4 - 3.5)*64 = 32, y = (3.5 - 4)*64 = -32
        let near_center2 = GridPos { x: 4, y: 4 };
        let world = grid_to_world(near_center2);
        assert_eq!(world.x, 0.5 * TILE_SIZE);
        assert_eq!(world.y, -0.5 * TILE_SIZE);
    }

    #[test]
    fn test_world_to_grid_roundtrip() {
        // Round-trip: convert grid to world, then back
        let positions = [
            GridPos { x: 0, y: 0 },
            GridPos { x: 7, y: 7 },
            GridPos { x: 1, y: 1 },
            GridPos { x: 6, y: 6 },
            GridPos { x: 3, y: 4 },
        ];
        for pos in positions {
            let world = grid_to_world(pos);
            let back = world_to_grid(world);
            assert_eq!(back, pos, "Round-trip failed for {:?}", pos);
        }
    }

    #[test]
    fn test_world_to_grid_bounds() {
        // World origin (0,0) is between 4 central tiles — rounds to (4,4) due to 3.5 rounding
        let grid = world_to_grid(Vec2::ZERO);
        assert_eq!(grid.x, 4);
        assert_eq!(grid.y, 4);

        // Exact tile centers
        let world = Vec2::new(-3.5 * TILE_SIZE, 3.5 * TILE_SIZE);
        assert_eq!(world_to_grid(world), GridPos { x: 0, y: 0 });

        let world = Vec2::new(3.5 * TILE_SIZE, -3.5 * TILE_SIZE);
        assert_eq!(world_to_grid(world), GridPos { x: 7, y: 7 });
    }

    #[test]
    fn test_build_room_walls() {
        let room = build_room(1);

        // Check perimeter walls
        for i in 0..8 {
            assert_eq!(room.get(i, 0), TileKind::Wall); // top row
            assert_eq!(room.get(i, 7), TileKind::Wall); // bottom row
            assert_eq!(room.get(0, i), TileKind::Wall); // left column
            assert_eq!(room.get(7, i), TileKind::Wall); // right column
        }

        // Check interior is floor (except exit)
        for x in 1..7 {
            for y in 1..7 {
                let tile = room.get(x, y);
                assert!(
                    tile == TileKind::Floor || tile == TileKind::Exit,
                    "Interior tile ({},{}) should be Floor or Exit, got {:?}",
                    x,
                    y,
                    tile
                );
            }
        }
    }

    #[test]
    fn test_exit_position_floor_progression() {
        // Floor 1: exit at (4, 4)
        let room1 = build_room(1);
        assert_eq!(room1.get(4, 4), TileKind::Exit);
        assert_eq!(room1.get(5, 4), TileKind::Floor);

        // Floor 2: exit at (5, 4)
        let room2 = build_room(2);
        assert_eq!(room2.get(5, 4), TileKind::Exit);
        assert_eq!(room2.get(4, 4), TileKind::Floor);

        // Floor 3: exit at (6, 4)
        let room3 = build_room(3);
        assert_eq!(room3.get(6, 4), TileKind::Exit);

        // Floor 4: exit wraps back to (4, 4) (4 + (3 % 3) = 4)
        let room4 = build_room(4);
        assert_eq!(room4.get(4, 4), TileKind::Exit);
    }
}
