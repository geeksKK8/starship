use bevy::prelude::*;

use crate::schedule::InGameSet;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

#[derive(Resource, Debug)]
pub struct Scoreboard {
    pub score: usize,
}

#[derive(Component, Debug)]
pub struct ScoreboardUi;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Scoreboard { score: 0 })
            .add_systems(Startup, spawn_scoreboard)
            .add_systems(Update, update_scoreboard.after(InGameSet::EntityUpdates));
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score:",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCORE_COLOR,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
        ScoreboardUi,
    ));
}
