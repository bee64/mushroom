use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<TextureAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/purpleMushroom.png")]
    pub purple_mushroom: Handle<Image>,

    #[asset(path = "textures/lol_sporeprint.png")]
    pub spore_print: Handle<Image>,

    #[asset(path = "textures/circle.png")]
    pub circle: Handle<Image>,
}
