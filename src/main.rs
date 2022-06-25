use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum State {
    Idle,
    Running,
    Paused,
    Over,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Action {
    Start,
    Pause,
    Resume,
    Reset,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Display {
    Poor,
    Good,
    Excellent
}
fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system()
        .add_system()
        .run();
}