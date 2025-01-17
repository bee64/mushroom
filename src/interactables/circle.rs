use crate::asset_loader::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct CirclePlugin;

#[derive(Component)]
pub struct Circle;

impl Plugin for CirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn);
    }
}

fn spawn(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Sprite::from_image(textures.circle.clone()),
        Transform::from_translation(Vec3::new(100.0, 100.0, 1.)),
        Circle,
    ));
    // .observe(|
    //     trigger: Trigger<Pointer<Drag>>,
    //     mut transforms: Query<&mut Transform>
    // |{
    //     let mut transform = transforms.get_mut(trigger.entity()).unwrap();
    //     let drag = trigger.event();
    //     transform.translation.x += drag.delta.x;
    //     transform.translation.y -= drag.delta.y;
    // });
}
