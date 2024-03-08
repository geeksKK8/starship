use bevy::prelude::*;

use crate::{
    asteroids::Asteroid, health::Health, schedule::InGameSet, scoreboard::Scoreboard,
    state::GameState,
};

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_far_away_entities, despawn_dead_entities).in_set(InGameSet::DespawnEntities),
        )
        .add_systems(OnEnter(GameState::GameOver), despawn_all_entities);
    }
}

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), With<Health>>,
) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        // Entity is far away from the camera's viewport.
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    query: Query<(Entity, &Health, Option<&Asteroid>)>,
) {
    for (entity, health, maybe_asteriod) in query.iter() {
        // Entity doesn't have any health.
        if health.value <= 0.0 {
            if maybe_asteriod.is_some() {
                scoreboard.score += 1;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<Entity, With<Health>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
