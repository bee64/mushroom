use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    let mut sprite = Sprite::from_image(textures.purpleMushroom.clone());
    sprite.custom_size = Some(Vec2::new(512.0, 512.0));
    commands.spawn((
        sprite,
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    for mut player_transform in &mut player_query {
        player_transform.rotate_z(f32::to_radians(360.0) * time.delta_secs())
    }
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 300.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_secs(),
        actions.player_movement.unwrap().y * speed * time.delta_secs(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
