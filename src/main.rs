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

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}


//DEFINED COORDINATES FOR THE GAME
const LEFT_BOUND: f32 = -450.0;
const RIGHT_BOUND: f32 = 450.0;
const TOP_BOUND: f32 = 300.0;
const BOTTOM_BOUND: f32 = -300.0;
const WALL_THICKNESS: f32 = 50.0;

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_BOUND, 0.),
            WallLocation::Right => Vec2::new(RIGHT_BOUND, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_BOUND),
            WallLocation::Top => Vec2::new(0., TOP_BOUND),
        }
    }
    fn size(&self) -> Vec2 {
        let arena_height = TOP_BOUND - BOTTOM_BOUND;
        let arena_width = RIGHT_BOUND - LEFT_BOUND;
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Right => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Bottom => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
            WallLocation::Top => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
        }
    }
}

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
   panic!("processed");
}