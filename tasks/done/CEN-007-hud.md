# CEN-007 — Steps Counter HUD

## Obiettivo
Creare un'interfaccia HUD minimalista che mostra in tempo reale i valori di Steps, Force e Floor number.

## Dipendenze
- CEN-004
- CEN-006

## Componenti / Risorse / Sistemi da Creare

### Components
```rust
#[derive(Component)]
pub struct HudStep;

#[derive(Component)]
pub struct HudForce;

#[derive(Component)]
pub struct HudFloor;

#[derive(Component)]
pub struct HudRoot;
```

### Plugin
- `HudPlugin`

### Systems
- `spawn_hud`: runs `OnEnter(GameState::Room)`, crea il container e i tre text nodes
- `update_steps_display`: runs `Update` `in_state(GameState::Room)`, aggiorna tutti i valori

## File da Creare / Modificare
- `src/ui/mod.rs` — plugin e re-export (nuovo)
- `src/ui/hud.rs` — HUD logic (nuovo)
- `src/plugins/mod.rs` — aggiungere `HudPlugin`

## Dettagli Implementativi

### HUD Layout
```
STEPS: 100
FORCE: 5
FLOOR: 1
```

Anchor: top-left corner, offset 16px da top, 16px da left.

### Text Properties
- Font size: 24px
- Color: `Color::WHITE`
- Background: transparent
- Font: default embedded font da Bevy

### Spawn Logic
```rust
fn spawn_hud(mut commands: Commands) {
    let root = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        HudRoot,
        StateScoped(GameState::Room),
    )).id();
    
    // Spawn STEPS text come figlio di root
    commands.entity(root).with_children(|parent| {
        parent.spawn((
            Text::new("STEPS: 100"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            HudStep,
        ));
        // etc. per Force e Floor
    });
}
```

### Update Logic
```rust
fn update_steps_display(
    player_query: Query<(&CurrentSteps, &Force), With<Player>>,
    config: Res<CenturionConfig>,
    mut steps_text: Query<&mut Text, With<HudStep>>,
    mut force_text: Query<&mut Text, With<HudForce>>,
    mut floor_text: Query<&mut Text, With<HudFloor>>,
) {
    if let Ok((current_steps, force)) = player_query.get_single() {
        for mut text in steps_text.iter_mut() {
            text.0 = format!("STEPS: {}", current_steps.0);
        }
        for mut text in force_text.iter_mut() {
            text.0 = format!("FORCE: {}", force.0);
        }
        for mut text in floor_text.iter_mut() {
            text.0 = format!("FLOOR: {}", config.current_floor);
        }
    }
}
```

## Criteri di Accettazione
- [ ] HUD è visibile in alto a sinistra all'entrata di `Room`
- [ ] HUD mostra "STEPS: 100", "FORCE: 5", "FLOOR: 1" al start
- [ ] Steps decrementano in tempo reale mentre il player si muove
- [ ] Force si aggiorna dopo ogni combattimento
- [ ] HUD scompare quando si esce da `Room` (grazie a `StateScoped`)
- [ ] I numeri aggiornano ogni frame senza lag visibile
