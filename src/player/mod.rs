use bevy::math::Vec3;
use bevy::prelude::{Commands, Component};

#[derive(Component)]
pub struct Player {
    pub position: Vec3,
    pub speed: f32,
    pub hp: i32,
    pub mana: i32,
    //TODO: skill
}

impl Player {
    fn run() {}
    fn spell_shot() {}
    fn setup() {}
}


pub fn register_player(mut commands: Commands) {
    commands.spawn(Player {
        position: Default::default(),
        speed: 10.0,
        hp: 10,
        mana: 10,
    });
}