use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod gamestate;
pub mod menu;
pub mod shooting;
pub mod strategy;
pub mod common;

use gamestate::{AppState, GameMode};
use menu::MenuPlugin;
use shooting::ShootingPlugin;
use strategy::StrategyPlugin;
use common::CommonPlugin;

fn main() {
    App::new()
        .init_state::<AppState>()
        .init_state::<GameMode>()
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .add_plugins(ShootingPlugin)
        .add_plugins(StrategyPlugin)
        .add_plugins(CommonPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}
