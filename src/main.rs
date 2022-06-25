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

struct Count{
    count: u32,
}

//DEFINED COORDINATES FOR THE GAME
const LEFT_BOUND: f32 = -450.0;
const RIGHT_BOUND: f32 = 450.0;
const TOP_BOUND: f32 = 300.0;
const BOTTOM_BOUND: f32 = -300.0;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(processed)
        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    //initalizing cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());


}
fn processed() {
   
}