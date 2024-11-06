use bevy::{
    color::palettes::basic::PURPLE, 
    prelude::*, 
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}};
use std::ops::Deref;


pub struct StreetNetworkPlugin;

impl Plugin for StreetNetworkPlugin {
    fn build(&self, app: &mut App){
        //app.add_systems(Startup, (spawn_streets));
        let streets: Vec<((u16, u16), (u16, u16))> = vec![
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
        app.insert_resource(available_streets(streets));
    }
}

pub const NUMBEROFSTORIES: u16 = 3;
pub const STREETLENGTH: u16 = 4;

#[derive(Copy, Clone)]
pub struct Node {
    pub coordinates: (u16,u16),
    pub world_coordinates: (f32, f32),
}

/*
#[derive(Bundle)]
pub struct NodeBundle<M: Material2d> {
    sprite: MaterialMesh2dBundle<M>,
    node: Node,
} */

#[derive(Component)]
pub struct Street //(((u16,u16),(u16,u16)));
{
    pub starting_node: Node,
    pub ending_node: Node
}

#[derive(Resource)]
pub struct available_streets(pub Vec<((u16, u16), (u16, u16))>);

/* 
impl Deref for Street {
    type Target = ((u16, u16), (u16, u16));

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}*/
/* 
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
}*/



/*pub fn spawn_streets(
    mut commands: Commands,
    window: Query<&Window>,
){
    //TODO: refactor this, so that Nodes get created separatly and copies into the streets and the street list contains of the nodes
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
    let window = window.single();
    for street in available_streets.iter(){
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
        commands.spawn(
            Street {
                starting_node,
                ending_node,
        });
    }
    
}*/