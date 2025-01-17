use crate::asset_loader::TextureAssets;
use crate::GameState;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

pub struct SporePrintPlugin;

#[derive(Component)]
pub struct SporePrint;

impl Plugin for SporePrintPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn);
    }
}

fn spawn(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Sprite::from_image(textures.spore_print.clone()),
        Transform::from_translation(Vec3::new(0.75, 0.75, 1.)),
        SporePrint,
    ))
    .observe(|
        trigger: Trigger<Pointer<Drag>>,
        mut transforms: Query<&mut Transform>
    |{
        // move the sprite
        let mut transform = transforms.get_mut(trigger.entity()).unwrap();
        let drag = trigger.event();
        transform.translation.x += drag.delta.x;
        transform.translation.y -= drag.delta.y;
    })
    .observe(|
        _trigger: Trigger<Pointer<DragEnd>>,
        mut transforms: Query<&mut Transform, With<Sprite>>
    |{
        let mut combos = transforms.iter_combinations_mut();
        while let Some([trans1, trans2]) = combos.fetch_next() {
            // TODO collision is pretty janky, hitboxes are very picky
            let collision = Aabb2d::new(trans1.translation.truncate(), trans1.scale.truncate() / 2.)
                .intersects(&Aabb2d::new(trans2.translation.truncate(), trans2.scale.truncate() / 2.));
            println!("thign!");
            
            if collision {
                println!("There was a collision!");
                // TODO set a timer and spawn a mushroom on collision
            }
        }
    });
}
