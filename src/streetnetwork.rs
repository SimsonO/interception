use bevy::{
    color::palettes::{basic::{PURPLE, RED, BLUE, GREEN, YELLOW}}, 
    prelude::*, 
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}};


pub struct StreetNetworkPlugin;

impl Plugin for StreetNetworkPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_network);
    }
}

const NUMBEROFSTORIES: u16 = 3;
const STREETLENGTH: u16 = 4;

#[derive(Component)]
pub struct Node {
    pub coordinates: (u16,u16),
}

#[derive(Bundle)]
pub struct NodeBundle<M: Material2d> {
    sprite: MaterialMesh2dBundle<M>,
    node: Node,
}

#[derive(Component)]
pub struct Street {
    pub nodes: (Node,Node),
}

fn spawn_network(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..STREETLENGTH {
        for j in 0..NUMBEROFSTORIES {
            let worldcoordinate = coordinate_to_worldcoordinates(&i,&j);
            commands.spawn(
                NodeBundle {
                    sprite: MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Circle::new(5.0))),
                        material: materials.add(Color::from(PURPLE)),
                        transform: Transform::from_xyz(worldcoordinate.0,worldcoordinate.1,0.0),
                        ..default()
                    },
                    node: Node {
                        coordinates: (i as u16, j as u16),
                    },
                });
        }
    }
}

fn coordinate_to_worldcoordinates(i:&u16, j:&u16) -> (f32, f32){
    let x: f32 = *i as f32 * 100.0;
    let y: f32 = *j as f32 * 100.0;
    (x,y)
}