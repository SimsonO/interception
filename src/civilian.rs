use bevy::{color::palettes::basic::BLUE, prelude::*};
use crate::streetnetwork::{NUMBEROFSTORIES, STREETLENGTH, Node, Street, available_streets} ;
use rand::Rng;


pub struct CivilianPlugin;

impl Plugin for CivilianPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_civs);
        app.add_systems(Update, move_civs);
    }
}
/* 
#[derive(Component)]
struct Person {
    current_street: (Node,Node),
    speed: f32,
    progress: f32, //defines where on the street the civ is
}*/

#[derive(Component, Deref)]
struct Speed(f32);
#[derive(Component, Deref)]
struct Progress(f32);

#[derive(Component)]
struct Civilian;

fn spawn_civs(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    streets: Res<available_streets>,
    window: Query<&Window>,
){ 
    let mut rng = rand::thread_rng();
    let window = window.single();
    let sprite_size = Some(Vec2::new(20.0,20.0));
    for street in streets.0.iter(){
        let world_coordinate_start = coordinate_to_world_coordinates(&street.0.0, &street.0.1, window);
        let starting_node = Node {
            coordinates: street.0,
            world_coordinates: world_coordinate_start,
        };
        let world_coordinate_end = coordinate_to_world_coordinates(&street.1.0, &street.1.1, window);
        let ending_node = Node {
            coordinates: street.1,
            world_coordinates: world_coordinate_end,
        };
        let street = Street {
            starting_node,
            ending_node
        };

        let number_of_civilians = rng.gen_range(10..=30);
        for _i in 0..number_of_civilians {
            let speed = rng.gen_range(1..4) as f32;
            let progress = rng.gen_range(0..=100) as f32 /100.0;
            let position = get_person_position(progress, &street);
            commands.spawn((
                Speed(speed),
                Progress(progress),
                Street{
                    starting_node,
                    ending_node
                },
                SpriteBundle {
                    texture: asset_server.load("characters/person.png"),
                    sprite: Sprite {
                        custom_size: sprite_size,
                        color: Color::from(BLUE),
                        ..default()
                    },
                    transform: Transform::from_xyz(position.0,position.1,0.0),
                    ..default()
                },
                Civilian)
            );
        }
   
    }
}

fn coordinate_to_world_coordinates(i:&u16, j:&u16, window: &Window) -> (f32, f32){
    let width = window.width();
    let height = window.height();
    let delta_top = height * 0.1;
    let x: f32 = 0.8 * width * (-0.5 + *i as f32 / (STREETLENGTH as f32 - 1.0));
    let y: f32 = 0.6 * height * (-0.5 + *j as f32 / (NUMBEROFSTORIES as f32 - 1.0)) - delta_top; 
    (x,y)
}

fn move_civs(
    mut civs: Query<(&Speed, &mut Progress, &mut Transform, &Street), With<Civilian>>,
    timer: Res<Time>
){
    for (speed, mut progress, mut transform, street) in &mut civs {
        progress.0 += speed.0 / 100.0 *  timer.delta_seconds();
        let new_position = get_person_position(progress.0, street);
        transform.translation = Vec3{x: new_position.0, y:  new_position.1, z: 0.0};
    }
}

fn get_person_position(progress: f32, street: &Street) -> (f32, f32){
    let x = street.starting_node.world_coordinates.0 * (1.0-progress) + street.ending_node.world_coordinates.0 * progress;
    let y = street.starting_node.world_coordinates.1 * (1.0-progress) + street.ending_node.world_coordinates.1 * progress;
    (x,y)
}