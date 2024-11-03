mod camera;
mod streetnetwork;

use bevy::prelude::*;
use camera::CameraPlugin;
use streetnetwork::StreetNetworkPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            StreetNetworkPlugin,
        ))
        .run();
}



