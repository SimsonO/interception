use bevy::{color::palettes::basic::BLUE, prelude::*};
use crate::streetnetwork::{Street, Node, spawn_streets} ;
use rand::Rng;


pub struct CivilianPlugin;

impl Plugin for CivilianPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_civs.after(spawn_streets));
    }
}
#[derive(Component)]
struct Person {
    current_street: ((u16,u16),(u16,u16)),
    speed: f32,
    progress: f32, //defines where on the street the civ is
}

#[derive(Component)]
struct Civilian;

fn spawn_civs(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    streets: Query<&Street>,
    nodes: Query<&Node>
){ 
    let mut rng = rand::thread_rng();
    let sprite_size = Some(Vec2::new(20.0,20.0));
    for street in &streets{
        let number_of_civilians = rng.gen_range(30..=100);
        for i in 0..number_of_civilians {
            let speed = rng.gen_range(1..4) as f32;
            let progress = rng.gen_range(0..=100) as f32 /100.0;
            if let Some(starting_node) = nodes.iter().find(|node|node.coordinates == street.0) {
                if let Some(ending_node) = nodes.iter().find(|node|node.coordinates == street.1) {
                    let starting_point: (f32, f32) = starting_node.world_coordinates;
                    let end_point = ending_node.world_coordinates;
                    let x = starting_point.0 * (1.0-progress) + end_point.0 * progress;
                    let y = starting_point.1 * (1.0-progress) + end_point.1 * progress;
                    commands.spawn((
                        Person {
                            current_street: **street,
                            speed,
                            progress,
                        },
                        SpriteBundle {
                            texture: asset_server.load("characters/person.png"),
                            sprite: Sprite {
                                custom_size: sprite_size,
                                color: Color::from(BLUE),
                                ..default()
                            },
                            transform: Transform::from_xyz(x,y,0.0),
                            ..default()
                        },
                        Civilian)
                    );
                }
            }
        }
    }
}