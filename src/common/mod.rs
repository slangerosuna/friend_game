use bevy::prelude::*;

pub mod networking;
use networking::NetworkingPlugin;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(NetworkingPlugin {app_id: 480, ..default()});
    }
}