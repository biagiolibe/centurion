# CEN-003 — Generatore Stanza 8x8 Deterministico

## Obiettivo
Creare una room deterministica 8x8, con muri sul perimetro, un'uscita, e tile floor; convertire in entità visibili. Nessun PRNG — la layout è derivata dal numero del piano.

## Dipendenze
- CEN-001
- CEN-002

## Componenti / Risorse / Sistemi da Creare

### Component
```rust
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPos {
    pub x: u8,
    pub y: u8,
}
```

### Enum
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileKind {
    Floor,
    Wall,
    Exit,
}
```

### Resource
```rust
#[derive(Resource)]
pub struct RoomLayout {
    tiles: [[TileKind; 8]; 8],
}

impl RoomLayout {
    pub fn get(&self, x: u8, y: u8) -> TileKind { ... }
}
```

### Plugin
- `MapGenPlugin`

### Functions
- `fn build_room(floor: u8) -> RoomLayout` — genera la layout per il piano dato
- `fn grid_to_world(pos: GridPos) -> Vec2` — converte GridPos a world coordinates
- `fn world_to_grid(pos: Vec2) -> GridPos` — converte back (con rounding)

### Systems
- `spawn_room`: runs `OnEnter(GameState::Room)`. Costruisce la room, crea entità sprite per ogni tile

## File da Creare / Modificare
- `src/map_gen/mod.rs` — plugin e re-export (nuovo)
- `src/map_gen/room.rs` — logica room generation (nuovo)
- `src/plugins/mod.rs` — aggiungere `MapGenPlugin`

## Dettagli Implementativi

### Constants
```rust
const TILE_SIZE: f32 = 64.0;
const GRID_WIDTH: u8 = 8;
const GRID_HEIGHT: u8 = 8;
const GRID_CENTER_X: f32 = 0.0;  // center della griglia in world space
const GRID_CENTER_Y: f32 = 0.0;
```

### Layout Generation
```rust
pub fn build_room(floor: u8) -> RoomLayout {
    let mut layout = [[TileKind::Floor; 8]; 8];
    
    // Muri sul perimetro (bordi)
    for x in 0..8 {
        layout[0][x] = TileKind::Wall;
        layout[7][x] = TileKind::Wall;
    }
    for y in 0..8 {
        layout[y][0] = TileKind::Wall;
        layout[y][7] = TileKind::Wall;
    }
    
    // Uscita a posizione derivata dal floor
    // Esempio: floor 1 → (4, 4); floor 2 → (5, 4); floor 3 → (6, 4), etc.
    let exit_x = 4 + ((floor - 1) % 3);
    let exit_y = 4;
    if exit_x < 8 && exit_y < 8 {
        layout[exit_y as usize][exit_x as usize] = TileKind::Exit;
    }
    
    RoomLayout { tiles: layout }
}
```

### Tile Rendering
- `Wall`: sprite quadrato (64x64) grigio scuro (es. `Color::rgb(0.3, 0.3, 0.3)`)
- `Floor`: sprite quadrato grigio leggermente più chiaro (es. `Color::rgb(0.5, 0.5, 0.5)`)
- `Exit`: sprite quadrato bianco (es. `Color::WHITE`) con un quadrato interno più scuro per indicare chiaramente che è una tile speciale

Ogni tile è un'entità `Sprite` con `GridPos`, `TileKind`, `Transform`, `StateScoped(GameState::Room)`

### Coordinate System
- World: (0, 0) al centro, Y aumenta verso l'alto (standard Bevy)
- Grid: (0, 0) top-left corner (come convenzione standard)
- Conversione:
  ```rust
  fn grid_to_world(pos: GridPos) -> Vec2 {
      Vec2::new(
          (pos.x as f32 - 3.5) * TILE_SIZE,
          (3.5 - pos.y as f32) * TILE_SIZE,
      )
  }
  ```

## Criteri di Accettazione
- [ ] La griglia 8x8 è visibile, centrata sullo schermo
- [ ] Muri su tutti e quattro i bordi (y=0, y=7, x=0, x=7)
- [ ] Un'unica uscita visibile per floor 1
- [ ] Uscita floor 1 è a (4, 4)
- [ ] Uscita floor 2 è a (5, 4)
- [ ] `grid_to_world` e `world_to_grid` sono testate in `#[cfg(test)]`
- [ ] Nessuna tile sovrapposta
- [ ] Tile non scompaiono / rimangono per tutta la durata di `Room` state
