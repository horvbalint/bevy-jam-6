//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    demo::{level::LevelAssets, player::PlayerAssets},
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Gameplay)
            .load_collection::<PlayerAssets>()
            .load_collection::<LevelAssets>(),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(Screen::Loading),
        children![widget::label("Loading...")],
    ));
}
