use bevy::{
    color::palettes::basic::PURPLE, 
    prelude::*, 
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}};
use std::ops::Deref;


pub struct StreetNetworkPlugin;

impl Plugin for StreetNetworkPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, (spawn_nodes, spawn_streets));
    }
}

const NUMBEROFSTORIES: u16 = 3;
const STREETLENGTH: u16 = 4;

#[derive(Component)]
pub struct Node {
    pub coordinates: (u16,u16),
    pub world_coordinates: (f32, f32),
}

#[derive(Bundle)]
pub struct NodeBundle<M: Material2d> {
    sprite: MaterialMesh2dBundle<M>,
    node: Node,
}

#[derive(Component)]
pub struct Street (((u16,u16),(u16,u16)));

impl Deref for Street {
    type Target = ((u16, u16), (u16, u16));

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn spawn_nodes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    for i in 0..STREETLENGTH {
        for j in 0..NUMBEROFSTORIES {
            let window = window.single();
            let world_coordinate = coordinate_to_world_coordinates(&i,&j, window);
            commands.spawn(
                NodeBundle {
                    sprite: MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Circle::new(5.0))),
                        material: materials.add(Color::from(PURPLE)),
                        transform: Transform::from_xyz(world_coordinate.0,world_coordinate.1,0.0),
                        ..default()
                    },
                    node: Node {
                        coordinates: (i as u16, j as u16),
                        world_coordinates: world_coordinate,
                    },
                });
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

pub fn spawn_streets(
    mut commands: Commands
){
    //TODO: might want to randomize this
    let available_streets: Vec<((u16, u16), (u16, u16))> = vec![
        ((0,0),(1,0)),
        ((1,0),(2,0)),
        ((2,0),(3,0)),
        ((0,1),(1,1)),
        ((1,1),(2,1)),
        ((2,1),(3,1)),
        ((0,2),(1,2)),
        ((1,2),(2,2)),
        ((2,2),(3,2)),
    ];
    for street in available_streets.iter(){
        commands.spawn(Street(*street));
    }
    
}