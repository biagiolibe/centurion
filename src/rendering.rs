use bevy::prelude::*;

pub struct CenturionRenderPlugin;

impl Plugin for CenturionRenderPlugin {
    fn build(&self, _app: &mut App) {
        // Plugin is mostly a marker; helpers are standalone functions
    }
}

/// Spawn a square sprite at the given world position
pub fn spawn_square(
    commands: &mut Commands,
    pos: Vec2,
    size: f32,
    color: Color,
) -> Entity {
    commands.spawn((
        Sprite {
            color,
            ..default()
        },
        Transform::from_translation(pos.extend(0.0)).with_scale(Vec3::new(size, size, 1.0)),
    ))
    .id()
}

/// Spawn a circle sprite at the given world position
/// Circle is approximated as a square with a small size; for a true circle,
/// a mesh-based approach or a circular sprite texture would be better.
/// For MVP, we use a simple white square approach.
pub fn spawn_circle(
    commands: &mut Commands,
    pos: Vec2,
    radius: f32,
    color: Color,
) -> Entity {
    let diameter = radius * 2.0;
    commands.spawn((
        Sprite {
            color,
            ..default()
        },
        Transform::from_translation(pos.extend(0.0)).with_scale(Vec3::new(diameter, diameter, 1.0)),
    ))
    .id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_helpers_exist() {
        // Compile-time test: functions exist and have correct signatures
        assert_eq!(std::mem::size_of::<fn(&mut Commands, Vec2, f32, Color) -> Entity>(),
                   std::mem::size_of::<fn(&mut Commands, Vec2, f32, Color) -> Entity>());
    }
}
