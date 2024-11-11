use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GameMode {
    #[default]
    NotStarted,
    Strategy,
    Shooter,
}
