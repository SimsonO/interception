mod camera;
mod streetnetwork;
mod civilian;

use bevy::prelude::*;
use camera::CameraPlugin;
use civilian::CivilianPlugin;
use streetnetwork::StreetNetworkPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            StreetNetworkPlugin,
            CivilianPlugin
        ))
        .run();
}



