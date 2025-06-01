//! The title screen that appears after the splash screen.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    menus::{Menu, credits::CreditsAssets},
    screens::Screen,
    theme::interaction::InteractionAssets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), open_main_menu);
    app.add_systems(OnExit(Screen::Title), close_menu);

    app.add_loading_state(
        LoadingState::new(Screen::Title)
            .load_collection::<InteractionAssets>()
            .load_collection::<CreditsAssets>(),
    );
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
