use crate::asset_loader::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PurpleMushroomPlugin;

#[derive(Component)]
pub struct PurpleMushroom;

impl Plugin for PurpleMushroomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn);
        // TODO watch for PurpleMushroom timer to complete
    }
}

fn spawn(mut commands: Commands, textures: Res<TextureAssets>) {
    // TODO: camera should probably be it's own plugin?
    commands.spawn((Camera2d, Msaa::Off));

    commands.spawn((
        Sprite::from_image(textures.purple_mushroom.clone()),
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        PurpleMushroom,
    ))
    .observe(|
        trigger: Trigger<Pointer<Drag>>,
        mut transforms: Query<&mut Transform>
    |{
        let mut transform = transforms.get_mut(trigger.entity()).unwrap();
        let drag = trigger.event();
        transform.translation.x += drag.delta.x;
        transform.translation.y -= drag.delta.y;
    });
}
