# CEN-001 — Foundation: App Bevy, Plugin Architecture, Rendering Geometrico

## Obiettivo
Stabilire l'infrastruttura Bevy di base con plugin architecture, helper per rendering geometrico (forme senza texture), e configurazione globale.

## Dipendenze
Nessuna

## Componenti / Risorse / Sistemi da Creare

### Resource
- `CenturionConfig`: contiene `current_floor: u8`, `window_width: f32`, `window_height: f32`

### Plugin
- `CenturionRenderPlugin`: registra le funzioni helper e configura lo stile di rendering

### Helper Functions (modulo `rendering`)
- `fn spawn_square(commands: &mut Commands, pos: Vec2, size: f32, color: Color) -> Entity`
- `fn spawn_circle(commands: &mut Commands, pos: Vec2, radius: f32, color: Color) -> Entity`

## File da Creare / Modificare
- `src/main.rs` — restructurato per plugin initialization
- `src/lib.rs` — esportazione moduli pubblici (nuovo)
- `src/plugins/mod.rs` — plugin group (nuovo)
- `src/rendering.rs` — helper geometrici (nuovo)
- `src/config.rs` — `CenturionConfig` resource (nuovo)

## Dettagli Implementativi

### Window Configuration
- Titolo: `"Centurion: 100 Steps"`
- Dimensioni: 800x800
- Background color: `Color::BLACK`

### `CenturionConfig`
```rust
#[derive(Resource)]
pub struct CenturionConfig {
    pub current_floor: u8,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for CenturionConfig {
    fn default() -> Self {
        Self {
            current_floor: 1,
            window_width: 800.0,
            window_height: 800.0,
        }
    }
}
```

### Plugin Group
```rust
pub struct CenturionPlugins;

impl PluginGroup for CenturionPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CenturionRenderPlugin)
            // altri plugin aggiunti in seguito
    }
}
```

### Helper Rendering
- `spawn_square` crea un `Sprite` rettangolare con `Transform`
- `spawn_circle` crea un `Sprite` (o un mesh?) circolare — preferibilmente sprite con un'immagine bianca semplice generata a runtime, oppure mesh geometrico semplice
- Entrambi piazzano l'entità a `pos` in world coordinates

## Criteri di Accettazione
- [ ] `cargo run` lancia una finestra 800x800 intitolata "Centurion: 100 Steps"
- [ ] Finestra ha background nero
- [ ] Un test system può invocare `spawn_square(commands, Vec2::ZERO, 64.0, Color::WHITE)` e vedere un quadrato bianco
- [ ] Un test system può invocare `spawn_circle(commands, Vec2::ZERO, 32.0, Color::WHITE)` e vedere un cerchio bianco
- [ ] Nessun file asset esterno è caricato (solo font embedded)
- [ ] `CenturionConfig` è registrata come resource con valori default
