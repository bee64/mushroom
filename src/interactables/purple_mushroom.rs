use crate::asset_loader::TextureAssets;
use bevy::prelude::*;

#[derive(Component)]
pub struct PurpleMushroom;

pub struct PurpleMushroomPlugin;
impl Plugin for PurpleMushroomPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_grow_mushroom);
    }
}

#[derive(Event)]
pub struct SpawnPurpleMushroom {
    pub position: Vec3
}
fn on_grow_mushroom(trigger: Trigger<SpawnPurpleMushroom>, mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Sprite::from_image(textures.purple_mushroom.clone()),
        Transform::from_translation(Vec3::new(trigger.position.x, trigger.position.y, trigger.position.z + 1.0)),
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
