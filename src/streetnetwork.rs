use bevy::prelude::*;

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
        ((1,0),(0,0)),
        ((3,0),(1,0)),
        ((3,0),(2,0)),
        ((1,1),(0,1)),
        ((2,1),(1,1)),
        ((3,1),(2,1)),
        ((1,2),(0,2)),
        ((2,2),(1,2)),
        ((3,2),(2,2)),
        ((0,0),(1,1)),
        ((0,1),(1,2)),
        ((1,2),(2,1)),
        ((2,0),(3,1)),
        ((3,1),(2,0)),
        ((1,1),(0,0)),
        ];
        app.insert_resource(AvailableStreets(streets));
    }
}

pub const NUMBEROFSTORIES: u16 = 3;
pub const STREETLENGTH: u16 = 4;

#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub coordinates: (u16,u16),
    pub world_coordinates: (f32, f32),
}

#[derive(Component, Debug)]
pub struct Street //(((u16,u16),(u16,u16)));
{
    pub starting_node: Node,
    pub ending_node: Node
}

#[derive(Resource)]
pub struct AvailableStreets(pub Vec<((u16, u16), (u16, u16))>);
