use crate::asset_loader::TextureAssets;
use crate::GameState;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use super::purple_mushroom::SpawnPurpleMushroom;

pub struct SporePrintPlugin;

#[derive(Component)]
pub struct SporePrint;

impl Plugin for SporePrintPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), spawn)
            .add_systems(Update, grow);
    }
}

#[derive(Component)]
pub struct SporeGrowthTimer {
    timer: Timer,
    position: Vec3,
}
fn grow (
    mut spore_growth_timers: Query<&mut SporeGrowthTimer>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for mut growth_timer in spore_growth_timers.iter_mut() {
        growth_timer.timer.tick(time.delta());

        if growth_timer.timer.just_finished() {
            commands.trigger(SpawnPurpleMushroom {
                position: growth_timer.position
            })
        }
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
        mut commands: Commands,
    |{
        let mut combos = transforms.iter_combinations_mut();
        while let Some([(trans1, sprite1), (trans2, sprite2)]) = combos.fetch_next() {
            let size1 = get_sprite_size(&assets, trans1, sprite1);
            let size2 = get_sprite_size(&assets, trans2, sprite2);
            let collision = Aabb2d::new(trans1.translation.truncate(), size1.half_size())
                .intersects(&Aabb2d::new(trans2.translation.truncate(), size2.half_size()));
            
            if collision {
                println!("There was a collision!");
                // start the spore growth timer
                commands.spawn( SporeGrowthTimer {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                    position: trans1.translation,
                });
            }
        }
    });
}

fn get_sprite_size(assets: &Res<Assets<Image>>, transform: &Transform, sprite: &Sprite) -> Rect {
    let image_size = &assets.get(&sprite.image).unwrap().size_f32();
    let scaled = image_size * transform.scale.truncate();
    return Rect::from_center_size(transform.translation.truncate(), scaled);
}
