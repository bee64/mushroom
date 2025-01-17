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
        mut transforms: Query<(&Transform, &Sprite), With<Sprite>>,
        assets: Res<Assets<Image>>,
    |{
        let mut combos = transforms.iter_combinations_mut();
        while let Some([(trans1, sprite1), (trans2, sprite2)]) = combos.fetch_next() {
            let size1 = get_sprite_size(&assets, trans1, sprite1);
            let size2 = get_sprite_size(&assets, trans2, sprite2);
            let collision = Aabb2d::new(trans1.translation.truncate(), size1.half_size())
                .intersects(&Aabb2d::new(trans2.translation.truncate(), size2.half_size()));
            
            if collision {
                println!("There was a collision!");
                // TODO set a timer and spawn a mushroom on collision
            }
        }
    });
}

// unscaled sprite size
fn get_sprite_size(assets: &Res<Assets<Image>>, transform: &Transform, sprite: &Sprite) -> Rect {
    let image_size = &assets.get(&sprite.image).unwrap().size_f32();
    let scaled = image_size * transform.scale.truncate();
    let bounding_box = Rect::from_center_size(transform.translation.truncate(), scaled);

    return bounding_box;
}
